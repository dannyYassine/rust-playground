use crate::{
    services::{ServiceProvider, ServiceRegistryBuilder},
    use_cases::{DecrementCounterUseCase, GetNameUseCase, IncrementCounterUseCase},
};
use async_trait::async_trait;

#[derive(Default)]
pub struct LoginServiceProvider;

#[async_trait]
impl ServiceProvider for LoginServiceProvider {
    async fn register(&self, service_registry: &mut ServiceRegistryBuilder) -> () {
        service_registry
            .register_type::<GetNameUseCase>()
            .register_type::<IncrementCounterUseCase>()
            .register_type::<DecrementCounterUseCase>();
    }
}
