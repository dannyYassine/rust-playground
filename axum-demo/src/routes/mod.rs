mod counter;
mod dashboard;
mod health;
mod htmx;
mod login;
mod root;
mod signup;
mod users;

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(root::router())
        .merge(users::router())
        .merge(health::router())
        .merge(htmx::router())
        .merge(login::router())
        .merge(signup::router())
        .merge(counter::router())
        .merge(dashboard::router())
}
