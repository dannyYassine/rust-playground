pub use component_derive::Component;
pub use component_derive::Injectable;

pub trait InlineAssets: askama::Template {
    fn css(&self) -> String {
        String::new()
    }
    fn js(&self) -> String {
        String::new()
    }
    fn component_name(&self) -> &'static str {
        ""
    }

    fn render_html(&self) -> askama::Result<String> {
        let markup = self.render()?;
        let css = self.css();
        let js = self.js();
        let name = self.component_name();

        let wrapped_css = if css.is_empty() {
            String::new()
        } else {
            format!("<style data-component=\"{}\">{}</style>", name, css)
        };

        let wrapped_js = if js.is_empty() {
            String::new()
        } else {
            format!("<script data-component=\"{}\">{}</script>", name, js)
        };

        Ok(format!("{}{}{}", wrapped_css, markup, wrapped_js))
    }
}

/// A wrapper type whose output is marked as HTML-safe (no escaping).
pub struct SafeHtml(pub String);

impl std::fmt::Display for SafeHtml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl askama::filters::HtmlSafe for SafeHtml {}

pub mod filters {
    use super::{InlineAssets, SafeHtml};

    #[askama::filter_fn]
    pub fn render<T: askama::Template + InlineAssets>(
        component: T,
        _values: &dyn askama::Values,
    ) -> askama::Result<SafeHtml> {
        Ok(SafeHtml(component.render_html()?))
    }
}
