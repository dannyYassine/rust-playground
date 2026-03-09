use std::{any::Any, sync::Arc};

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
};
use tokio::sync::Mutex;

mod app;
mod db;
mod models;
mod requests;
mod routes;
mod services;
mod templates;
mod use_cases;

use services::{NewFromContainer, ServiceRegistry};

use crate::{
    db::DatabaseServiceProvider,
    models::User,
    services::{
        ApplicationServiceProvider, Extract, LoginServiceProvider, ServiceProvider,
        UserServiceProvider,
    },
    use_cases::{UserRepo, UserRepository},
};

struct AppState {
    count: Mutex<isize>,
    registry: Arc<ServiceRegistry>,
}

impl AsRef<ServiceRegistry> for Arc<AppState> {
    fn as_ref(&self) -> &ServiceRegistry {
        &self.registry
    }
}

impl<S, T> FromRequestParts<S> for Extract<T>
where
    S: AsRef<ServiceRegistry> + Send + Sync,
    T: Any + Send + Sync + NewFromContainer + 'static,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Extract(state.as_ref().get_or_new::<T>()))
    }
}

impl<S> FromRequestParts<S> for User
where
    S: AsRef<ServiceRegistry> + Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    // Use the standard signature without manual lifetime labels
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get(header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "No cookies found"))?;

        let user_id = cookie_header
            .split(';')
            .map(|s| s.trim())
            .find_map(|s| {
                let mut p = s.splitn(2, '=');
                let (key, val) = (p.next()?, p.next()?);
                if key == "user_id" {
                    Some(val.to_string())
                } else {
                    None
                }
            })
            .ok_or((StatusCode::UNAUTHORIZED, "user_id cookie missing"))?
            .parse::<i32>()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user_id format"))?;

        let registry: &ServiceRegistry = state.as_ref();
        let user_repo = registry.get_or_new::<UserRepo>();

        user_repo
            .find_by_id(user_id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let count = Mutex::new(0);
    let mut registry_builder = ServiceRegistry::builder();

    let mut application_service_provider: ApplicationServiceProvider =
        ApplicationServiceProvider::new();
    application_service_provider
        .add_provider::<DatabaseServiceProvider>()
        .add_provider::<LoginServiceProvider>()
        .add_provider::<UserServiceProvider>()
        .register(&mut registry_builder)
        .await;

    let registry = Arc::new(registry_builder.build());
    let shared_state = Arc::new(AppState { count, registry });

    tracing_subscriber::fmt::init();

    let app = routes::router().with_state(shared_state);

    let host = std::env::var("HOST").expect("HOST should be in the env.");
    let port = std::env::var("PORT").expect("PORT should be in the env.");

    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://localhost:{}", port);

    application_service_provider.boot().await;
    let server = axum::serve(listener, app).await;
    server.unwrap();
}
