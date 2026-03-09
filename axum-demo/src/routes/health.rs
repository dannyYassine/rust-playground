use std::sync::Arc;

use axum::Router;

use crate::{AppState, services::Route};

pub fn router() -> Router<Arc<AppState>> {
    Route::new().get("/health", || async { "OK" }).into_router()
}
