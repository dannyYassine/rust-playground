use std::sync::Arc;

use axum::Form;
use axum::{Router, http::StatusCode};

use crate::AppState;
use crate::requests::SignupRequest;
use crate::services::{Extract, Route, SignUpPresenter};
use crate::templates::SignupTemplate;
use crate::use_cases::SignUpUseCase;

pub fn router() -> Router<Arc<AppState>> {
    Route::new()
        .get("/signup", signup_page_handler)
        .post("/htmx/signup", signup_form_handler)
        .into_router()
}

async fn signup_page_handler() -> Result<SignupTemplate, StatusCode> {
    return Ok(SignupTemplate::new());
}

async fn signup_form_handler(
    Extract(use_case): Extract<SignUpUseCase>,
    Form(payload): Form<SignupRequest>,
) -> SignUpPresenter {
    return SignUpPresenter(
        use_case
            .execute(
                &payload.name,
                &payload.email,
                &payload.password,
                &payload.confirm_password,
            )
            .await,
    );
}
