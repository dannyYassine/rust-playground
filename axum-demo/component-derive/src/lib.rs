use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(component))]
struct ComponentArgs {
    ident: syn::Ident,
    generics: syn::Generics,

    /// Inline CSS string embedded directly in the attribute.
    #[darling(default)]
    css: Option<String>,

    /// Path to a CSS file, relative to the crate root (loaded at compile time).
    #[darling(default)]
    css_path: Option<String>,

    /// Inline JS string embedded directly in the attribute.
    #[darling(default)]
    js: Option<String>,

    /// Path to a JS file, relative to the crate root (loaded at compile time).
    #[darling(default)]
    js_path: Option<String>,
}

/// # Component Derive Macro
///
/// Generates an `InlineAssets` impl so CSS/JS gets injected when rendered via the `render` filter.
///
/// ## Inline assets (embedded in the attribute)
/// ```rust
/// #[derive(Template, Component)]
/// #[template(path = "components/counter.html")]
/// #[component(css = "#counter { color: blue; }", js = "console.log('hi');")]
/// pub struct Counter { count: isize }
/// ```
///
/// ## File-based assets (loaded at compile time)
/// ```rust
/// #[derive(Template, Component)]
/// #[template(path = "components/card.html")]
/// #[component(css_path = "static/card.css", js_path = "static/card.js")]
/// pub struct Card { title: String }
/// ```
///
/// ## Both inline and file-based (merged together)
/// ```rust
/// #[derive(Template, Component)]
/// #[template(path = "components/alert.html")]
/// #[component(css = ".alert { color: red; }", css_path = "static/alert-base.css")]
/// pub struct Alert { variant: String }
/// ```
///
#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let args = match ComponentArgs::from_derive_input(&input) {
        Ok(args) => args,
        Err(e) => return e.write_errors().into(),
    };

    let name = &args.ident;
    let (impl_generics, ty_generics, where_clause) = args.generics.split_for_impl();
    let component_name = to_snake_case(&name.to_string());

    let inline_css = args.css.as_deref().unwrap_or("");
    let inline_js = args.js.as_deref().unwrap_or("");

    let file_css = match &args.css_path {
        Some(path) => quote! {
            const FILE_CSS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #path));
        },
        None => quote! {
            const FILE_CSS: &str = "";
        },
    };

    let file_js = match &args.js_path {
        Some(path) => quote! {
            const FILE_JS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #path));
        },
        None => quote! {
            const FILE_JS: &str = "";
        },
    };

    let expanded = quote! {
        impl #impl_generics component::InlineAssets for #name #ty_generics #where_clause {
            fn css(&self) -> String {
                #file_css
                let mut out = FILE_CSS.to_string();
                if !#inline_css.is_empty() {
                    if !out.is_empty() { out.push('\n'); }
                    out.push_str(#inline_css);
                }
                out
            }
            fn js(&self) -> String {
                #file_js
                let mut out = FILE_JS.to_string();
                if !#inline_js.is_empty() {
                    if !out.is_empty() { out.push('\n'); }
                    out.push_str(#inline_js);
                }
                out
            }
            fn component_name(&self) -> &'static str {
                #component_name
            }
        }
    };

    TokenStream::from(expanded)
}

/// # Injectable Derive Macro
///
/// Generates `NewFromContainer` (and `Default` for no-field structs) by inspecting
/// each field's type and optional `#[inject(...)]` attribute:
///
/// - **No fields** — generates `Default` + `new_from_container` that calls `Self::default()`
/// - **`field: SomeStruct`** — calls `SomeStruct::new_from_container(registry)`
/// - **`field: Arc<SomeStruct>`** — calls `registry.get_or_new::<SomeStruct>()`
/// - **`#[inject(registered)] field: Arc<SomeStruct>`** — calls `registry.get::<SomeStruct>().expect(...)`;
///   use this for third-party types (e.g. `PgPool`) that are pre-registered but don't impl `NewFromContainer`
/// - **`field: Arc<dyn Trait>`** — no impl generated; implement `NewFromContainer` manually
///
/// ```rust
/// #[derive(Injectable)]
/// pub struct MyUseCase {
///     repo: Arc<UserRepo>,
/// }
///
/// #[derive(Injectable)]
/// pub struct UserRepo {
///     #[inject(registered)]
///     pool: Arc<PgPool>,
/// }
/// ```
#[proc_macro_derive(Injectable, attributes(inject))]
pub fn derive_injectable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => {
            return quote! { compile_error!("#[derive(Injectable)] only supports structs"); }.into()
        }
    };

    let named = match fields {
        Fields::Named(f) => &f.named,
        Fields::Unit => {
            // No fields — generate Default + trivial new_from_container
            let expanded = quote! {
                impl #impl_generics Default for #name #ty_generics #where_clause {
                    fn default() -> Self { Self {} }
                }
                impl #impl_generics crate::services::NewFromContainer for #name #ty_generics #where_clause {
                    fn new_from_container(_: &crate::services::ServiceRegistry) -> Self {
                        Self::default()
                    }
                }
            };
            return TokenStream::from(expanded);
        }
        Fields::Unnamed(_) => {
            return quote! { compile_error!("#[derive(Injectable)] does not support tuple structs"); }
                .into()
        }
    };

    // Named fields — build each field initializer
    let mut field_inits: Vec<TokenStream2> = Vec::new();

    for field in named {
        let field_name = field.ident.as_ref().unwrap();
        let kind = if has_inject_registered(field) {
            match classify_type(&field.ty) {
                FieldKind::ArcConcrete(inner) => FieldKind::ArcRegistered(inner),
                _ => {
                    return quote! {
                        compile_error!("#[inject(registered)] is only valid on Arc<ConcreteType> fields");
                    }
                    .into()
                }
            }
        } else {
            classify_type(&field.ty)
        };

        match kind {
            FieldKind::Plain => {
                let ty = &field.ty;
                field_inits.push(quote! {
                    #field_name: <#ty as crate::services::NewFromContainer>::new_from_container(registry)
                });
            }
            FieldKind::ArcConcrete(inner) => {
                field_inits.push(quote! {
                    #field_name: registry.get_or_new::<#inner>()
                });
            }
            FieldKind::ArcRegistered(inner) => {
                let expect_msg = format!(
                    "{} should be registered in the ServiceRegistry",
                    quote!(#inner)
                );
                field_inits.push(quote! {
                    #field_name: registry.get::<#inner>().expect(#expect_msg)
                });
            }
            FieldKind::Unsupported => {
                // Cannot auto-generate — user must implement NewFromContainer manually.
                return TokenStream::new();
            }
        }
    }

    // Named-field struct — no Default generated (fields may not be Default)
    let expanded = quote! {
        impl #impl_generics crate::services::NewFromContainer for #name #ty_generics #where_clause {
            fn new_from_container(registry: &crate::services::ServiceRegistry) -> Self {
                Self {
                    #(#field_inits),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

enum FieldKind {
    /// Plain concrete type: `SomeStruct`
    Plain,
    /// Arc-wrapped concrete auto-constructed: `Arc<SomeStruct>` via get_or_new
    ArcConcrete(Type),
    /// Arc-wrapped type fetched from registry only: `Arc<SomeStruct>` via get + expect
    ArcRegistered(Type),
    /// Arc<dyn Trait> or other unsupported form
    Unsupported,
}

fn has_inject_registered(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("inject")
            && attr
                .parse_args::<syn::Ident>()
                .map(|id| id == "registered")
                .unwrap_or(false)
    })
}

fn classify_type(ty: &Type) -> FieldKind {
    let Type::Path(type_path) = ty else {
        return FieldKind::Plain;
    };

    let last = match type_path.path.segments.last() {
        Some(s) => s,
        None => return FieldKind::Plain,
    };

    if last.ident != "Arc" {
        return FieldKind::Plain;
    }

    let PathArguments::AngleBracketed(args) = &last.arguments else {
        return FieldKind::Unsupported;
    };

    let Some(GenericArgument::Type(inner)) = args.args.first() else {
        return FieldKind::Unsupported;
    };

    match inner {
        Type::TraitObject(_) => FieldKind::Unsupported,
        other => FieldKind::ArcConcrete(other.clone()),
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}
