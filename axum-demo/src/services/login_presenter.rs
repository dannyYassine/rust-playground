use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::models::User;
use crate::{
    templates::{FlashTemplate, TemplateCollection},
    use_cases::LoginError,
};

pub struct LoginPresenter(pub Result<User, LoginError>);

impl IntoResponse for LoginPresenter {
    fn into_response(self) -> Response {
        return match self.0 {
            Ok(user) => {
                let mut headers = HeaderMap::new();
                headers.insert("HX-Redirect", "/dashboard".parse().unwrap());
                headers.insert(
                    "Set-Cookie",
                    format!("user_id={}; Path=/; HttpOnly", user.id)
                        .parse()
                        .unwrap(),
                );
                (StatusCode::OK, headers).into_response()
            }
            Err(err) => TemplateCollection::new()
                .add(FlashTemplate {
                    error: Some(err.to_string()),
                    success: None,
                })
                .into_response(),
        };
    }
}
