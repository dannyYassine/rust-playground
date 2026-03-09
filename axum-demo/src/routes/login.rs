use std::sync::Arc;

use axum::Form;
use axum::{Router, http::StatusCode};

use crate::AppState;
use crate::requests::LoginRequest;
use crate::services::{Extract, LoginPresenter, Route};
use crate::templates::LoginTemplate;
use crate::use_cases::LoginUseCase;

pub fn router() -> Router<Arc<AppState>> {
    Route::new()
        .get("/login", login_page_handler)
        .post("/htmx/login", login_form_handler)
        .into_router()
}

async fn login_page_handler() -> Result<LoginTemplate, StatusCode> {
    return Ok(LoginTemplate::new());
}

async fn login_form_handler(
    Extract(use_case): Extract<LoginUseCase>,
    Form(payload): Form<LoginRequest>,
) -> LoginPresenter {
    return LoginPresenter(use_case.execute(&payload.email, &payload.password).await);
}
