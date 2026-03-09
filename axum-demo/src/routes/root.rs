use std::sync::Arc;

use axum::extract::State;
use axum::{Router, extract::Request};

use crate::AppState;
use crate::requests::HtmlRequest;
use crate::services::Route;
use crate::templates::HelloTemplate;
use crate::use_cases::GetNameUseCase;

pub fn router() -> Router<Arc<AppState>> {
    Route::new()
        .get("/", || async { "Hello from Axum!" })
        .get(
            "/html",
            |State(state): State<Arc<AppState>>, request: Request| async move {
                let count: isize;
                {
                    let mut counter = state.count.lock().await;
                    *counter += 1;

                    count = counter.clone();
                }

                let params_string = request.uri().query().unwrap_or("");
                let params: HtmlRequest =
                    serde_urlencoded::from_str(params_string).unwrap_or_default();

                let name = state
                    .registry
                    .get::<GetNameUseCase>()
                    .expect("GetNameUseCase missing")
                    .execute(params.name)
                    .await;

                HelloTemplate { name, count }
            },
        )
        .into_router()
}
