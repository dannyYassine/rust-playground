use crate::{
    services::{ServiceProvider, ServiceRegistryBuilder},
    use_cases::UserRepo,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[derive(Default)]
pub struct UserServiceProvider;

#[async_trait]
impl ServiceProvider for UserServiceProvider {
    async fn register(&self, service_registry: &mut ServiceRegistryBuilder) -> () {
        service_registry.register_type::<UserRepo>();
        // service_registry.register_factory::<UserRepo>(move |s| {
        //     UserRepo::new(
        //         s.get::<Pool<Postgres>>()
        //             .clone()
        //             .expect("Postgres should be there"),
        //     )
        // });
    }
}
