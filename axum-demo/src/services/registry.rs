use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Entry
// ---------------------------------------------------------------------------

/// How a service is stored in the registry.
enum Entry {
    /// A pre-built instance shared across all `get` calls.
    Singleton(Arc<dyn Any + Send + Sync>),

    /// A factory invoked on every `get` call, producing a fresh instance each time.
    Transient(Box<dyn Fn(&ServiceRegistry) -> Arc<dyn Any + Send + Sync> + Send + Sync>),
}

pub trait NewFromContainer {
    fn new_from_container(registry: &ServiceRegistry) -> Self;
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// An immutable, type-safe service registry for storing usecase structs.
///
/// Built once at startup via [`ServiceRegistryBuilder`], then shared across
/// threads as `Arc<ServiceRegistry>` with no locking overhead on reads.
///
/// Two registration modes are supported:
///
/// - **Singleton** — one instance is built at registration time and shared on
///   every `get` call (see [`ServiceRegistryBuilder::register`] and
///   [`ServiceRegistryBuilder::register_arc`]).
///
/// - **Transient** — no instance is built at registration time; a fresh one is
///   constructed on every `get` call via the provided factory closure (see
///   [`ServiceRegistryBuilder::register_type`]).
///
/// # Example
///
/// ```rust
/// let registry = Arc::new(
///     ServiceRegistry::builder()
///         .register(CreateUserUseCase::new())   // singleton
///         .register_type::<DeleteUserUseCase>() // transient (requires Default)
///         .build(),
/// );
///
/// // Returns the *same* Arc every time:
/// let create = registry.get::<CreateUserUseCase>().unwrap();
///
/// // Returns a *new* instance every time:
/// let delete = registry.get::<DeleteUserUseCase>().unwrap();
/// ```
pub struct ServiceRegistry {
    entries: HashMap<TypeId, Entry>,
}

impl ServiceRegistry {
    /// Create a new [`ServiceRegistryBuilder`].
    pub fn builder() -> ServiceRegistryBuilder {
        ServiceRegistryBuilder::new()
    }

    /// Retrieve a service by its concrete type.
    ///
    /// - **Singleton** entries return a clone of the shared `Arc<T>`.
    /// - **Transient** entries construct and return a brand-new `Arc<T>`.
    ///
    /// Returns `None` if the type was never registered.
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        match self.entries.get(&TypeId::of::<T>())? {
            Entry::Singleton(arc) => arc.clone().downcast::<T>().ok(),
            Entry::Transient(factory) => factory(self).downcast::<T>().ok(),
        }
    }

    /// Retrieve a service by type, or construct it via `NewFromContainer` if not registered.
    pub fn get_or_new<T: Any + Send + Sync + NewFromContainer + 'static>(&self) -> Arc<T> {
        self.get::<T>()
            .unwrap_or_else(|| Arc::new(T::new_from_container(self)))
    }

    /// Returns `true` if a service of type `T` has been registered (either
    /// as a singleton or as a transient factory).
    #[allow(dead_code)]
    pub fn has<T: Any + Send + Sync + 'static>(&self) -> bool {
        self.entries.contains_key(&TypeId::of::<T>())
    }

    /// Returns the total number of registered entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if no services have been registered.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// SAFETY: Every value inside `Entry` is either `Arc<dyn Any + Send + Sync>`
// or a `Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>`, both of
// which are Send + Sync.
//
// unsafe impl Send for ServiceRegistry {}
// unsafe impl Sync for ServiceRegistry {}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Fluent builder for constructing a [`ServiceRegistry`].
pub struct ServiceRegistryBuilder {
    entries: HashMap<TypeId, Entry>,
}

impl Default for ServiceRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistryBuilder {
    /// Create a new, empty builder.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Register a pre-built singleton instance.
    ///
    /// The value is stored in an `Arc` and the *same* `Arc` is returned on
    /// every `get::<T>()` call.
    ///
    /// If a service of the same type was already registered, it is replaced.
    pub fn register<T: Any + Send + Sync + 'static>(&mut self, service: T) -> &mut Self {
        self.entries
            .insert(TypeId::of::<T>(), Entry::Singleton(Arc::new(service)));
        self
    }

    /// Register a pre-built singleton that is already wrapped in an `Arc`.
    ///
    /// Useful when the same `Arc` instance must be shared with other parts of
    /// the application outside of the registry.
    #[allow(dead_code)]
    pub fn register_arc<T: Any + Send + Sync + 'static>(&mut self, service: Arc<T>) -> &mut Self {
        self.entries
            .insert(TypeId::of::<T>(), Entry::Singleton(service));
        self
    }

    /// Register a type as a **transient** factory.
    ///
    /// Nothing is built at registration time. Instead, `T::default()` is
    /// called on every `get::<T>()` invocation, returning a fresh `Arc<T>`
    /// each time.
    ///
    /// Requires `T: Default`. For types without `Default`, use
    /// [`register_factory`](Self::register_factory) and supply a closure.
    pub fn register_type<T: Any + Send + Sync + NewFromContainer + 'static>(
        &mut self,
    ) -> &mut Self {
        let factory: Box<dyn Fn(&ServiceRegistry) -> Arc<dyn Any + Send + Sync> + Send + Sync> =
            Box::new(|s| Arc::new(T::new_from_container(s)));
        self.entries
            .insert(TypeId::of::<T>(), Entry::Transient(factory));
        self
    }

    /// Register a custom factory closure as a **transient** entry.
    ///
    /// The closure is called on every `get::<T>()` invocation, producing a
    /// fresh instance each time. Use this when the type does not implement
    /// `Default` or when construction requires arguments captured in the
    /// closure.
    ///
    /// ```rust
    /// .register_factory(|| MyUseCase::new(dependency.clone()))
    /// ```
    pub fn register_factory<T>(
        &mut self,
        factory: impl Fn(&ServiceRegistry) -> T + Send + Sync + 'static,
    ) -> &mut Self
    where
        T: Any + Send + Sync + 'static,
    {
        let factory: Box<dyn Fn(&ServiceRegistry) -> Arc<dyn Any + Send + Sync> + Send + Sync> =
            Box::new(move |s| Arc::new(factory(s)));
        self.entries
            .insert(TypeId::of::<T>(), Entry::Transient(factory));
        self
    }

    /// Consume the builder and return an immutable [`ServiceRegistry`].
    pub fn build(self) -> ServiceRegistry {
        ServiceRegistry {
            entries: self.entries,
        }
    }

    /// Consume the builder by value, useful for call-site ergonomics when
    /// chaining from an owned `ServiceRegistryBuilder`.
    #[allow(dead_code)]
    pub fn build_from(builder: Self) -> ServiceRegistry {
        builder.build()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[derive(Default)]
    struct FooUseCase {
        pub value: u32,
    }
    impl NewFromContainer for FooUseCase {
        fn new_from_container(_registry: &ServiceRegistry) -> Self {
            Self::default()
        }
    }

    #[derive(Default)]
    struct BarUseCase {
        pub label: String,
    }
    impl NewFromContainer for BarUseCase {
        fn new_from_container(_registry: &ServiceRegistry) -> Self {
            Self::default()
        }
    }

    // Counts how many times it has been constructed.
    static BUILD_COUNT: AtomicU32 = AtomicU32::new(0);

    #[derive(Default)]
    struct CountedUseCase;

    impl CountedUseCase {
        fn new() -> Self {
            BUILD_COUNT.fetch_add(1, Ordering::SeqCst);
            Self
        }
    }

    // ---- singleton ----------------------------------------------------------

    #[test]
    fn singleton_register_and_get() {
        let registry = ServiceRegistry::builder()
            .register(FooUseCase { value: 42 })
            .build();

        let foo = registry.get::<FooUseCase>().expect("should be registered");
        assert_eq!(foo.value, 42);
    }

    #[test]
    fn singleton_returns_same_arc() {
        let registry = ServiceRegistry::builder()
            .register(FooUseCase { value: 1 })
            .build();

        let a = registry.get::<FooUseCase>().unwrap();
        let b = registry.get::<FooUseCase>().unwrap();
        assert!(Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn register_arc_preserves_pointer() {
        let arc = Arc::new(FooUseCase { value: 99 });
        let arc_clone = Arc::clone(&arc);

        let registry = ServiceRegistry::builder().register_arc(arc).build();

        let retrieved = registry.get::<FooUseCase>().unwrap();
        assert!(Arc::ptr_eq(&retrieved, &arc_clone));
    }

    // ---- transient (register_type) ------------------------------------------

    #[test]
    fn transient_register_type_and_get() {
        let registry = ServiceRegistry::builder()
            .register_type::<BarUseCase>()
            .build();

        let bar = registry.get::<BarUseCase>().expect("should be registered");
        assert_eq!(bar.label, "");
    }

    #[test]
    fn transient_returns_different_arcs() {
        let registry = ServiceRegistry::builder()
            .register_type::<BarUseCase>()
            .build();

        let a = registry.get::<BarUseCase>().unwrap();
        let b = registry.get::<BarUseCase>().unwrap();
        // Each call must produce a distinct allocation.
        assert!(!Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn transient_builds_on_every_get() {
        BUILD_COUNT.store(0, Ordering::SeqCst);

        let registry = ServiceRegistry::builder()
            .register_factory(|_| CountedUseCase::new())
            .build();

        registry.get::<CountedUseCase>().unwrap();
        registry.get::<CountedUseCase>().unwrap();
        registry.get::<CountedUseCase>().unwrap();

        assert_eq!(BUILD_COUNT.load(Ordering::SeqCst), 3);
    }

    // ---- register_factory ---------------------------------------------------

    #[test]
    fn register_factory_captures_context() {
        let base = 100u32;

        let registry = ServiceRegistry::builder()
            .register_factory(move |_| FooUseCase { value: base + 1 })
            .build();

        let foo = registry.get::<FooUseCase>().unwrap();
        assert_eq!(foo.value, 101);
    }

    // ---- shared helpers -----------------------------------------------------

    #[test]
    fn get_unregistered_returns_none() {
        let registry = ServiceRegistry::builder().build();
        assert!(registry.get::<FooUseCase>().is_none());
    }

    #[test]
    fn has_works_for_both_variants() {
        let registry = ServiceRegistry::builder()
            .register(FooUseCase { value: 0 })
            .register_type::<BarUseCase>()
            .build();

        assert!(registry.has::<FooUseCase>());
        assert!(registry.has::<BarUseCase>());
    }

    #[test]
    fn len_and_is_empty() {
        let empty = ServiceRegistry::builder().build();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let two = ServiceRegistry::builder()
            .register(FooUseCase { value: 0 })
            .register_type::<BarUseCase>()
            .build();

        assert!(!two.is_empty());
        assert_eq!(two.len(), 2);
    }

    #[test]
    fn later_registration_replaces_earlier() {
        let registry = ServiceRegistry::builder()
            .register(FooUseCase { value: 1 })
            .register(FooUseCase { value: 2 })
            .build();

        assert_eq!(registry.get::<FooUseCase>().unwrap().value, 2);
    }

    #[test]
    fn is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ServiceRegistry>();
        assert_send_sync::<Arc<ServiceRegistry>>();
    }
}
