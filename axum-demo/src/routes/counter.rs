use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, response::Html};
use component::InlineAssets;

use crate::AppState;
use crate::services::{CounterPresenter, Extract, Route};
use crate::templates::CounterPageTemplate;
use crate::use_cases::{DecrementCounterUseCase, IncrementCounterUseCase};

pub fn router() -> Router<Arc<AppState>> {
    Route::new()
        .get("/counter", counter_page_handler)
        .post("/htmx/counter/add", increment_counter_handler)
        .post("/htmx/counter/substract", decrement_counter_handler)
        .into_router()
}

async fn counter_page_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let count = *state.count.lock().await;
    let template = CounterPageTemplate::new(count);
    let html = template
        .render_html()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(html))
}

async fn increment_counter_handler(
    Extract(use_case): Extract<IncrementCounterUseCase>,
    State(state): State<Arc<AppState>>,
) -> CounterPresenter {
    let count = use_case.execute(&state.count).await;
    return CounterPresenter(count);
}

async fn decrement_counter_handler(
    Extract(use_case): Extract<DecrementCounterUseCase>,
    State(state): State<Arc<AppState>>,
) -> CounterPresenter {
    let count = use_case.execute(&state.count).await;
    return CounterPresenter(count);
}
