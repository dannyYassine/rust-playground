use crate::{
    db::{self, database::refresh_database},
    services::{ServiceProvider, ServiceRegistryBuilder},
};
use async_trait::async_trait;

#[derive(Default)]
pub struct DatabaseServiceProvider {}

#[async_trait]
impl ServiceProvider for DatabaseServiceProvider {
    async fn register(&self, service_registry: &mut ServiceRegistryBuilder) {
        // db::database::refresh_database().await;

        let pool = db::database::create_pool().await;
        service_registry.register(pool);
    }
}
