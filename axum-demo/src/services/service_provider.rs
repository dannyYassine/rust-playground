use crate::services::ServiceRegistryBuilder;
use async_trait::async_trait;

#[async_trait]
pub trait ServiceProvider {
    async fn register(&self, service_registry: &mut ServiceRegistryBuilder);
    async fn boot(&self) {}
}

#[derive(Default)]
pub struct ApplicationServiceProvider {
    service_provider: Vec<Box<dyn ServiceProvider + Send + Sync>>,
}
impl ApplicationServiceProvider {
    pub fn new() -> Self {
        Self {
            service_provider: Vec::new(),
        }
    }

    pub fn add_provider<T>(&mut self) -> &mut Self
    where
        T: ServiceProvider + Default + Send + Sync + 'static,
    {
        self.service_provider.push(Box::new(T::default()));

        return self;
    }
}

#[async_trait]
impl ServiceProvider for ApplicationServiceProvider {
    async fn register(&self, app: &mut ServiceRegistryBuilder) -> () {
        for provider in &self.service_provider {
            provider.register(app).await;
        }
    }

    async fn boot(&self) -> () {
        for provider in &self.service_provider {
            provider.boot().await;
        }
    }
}
