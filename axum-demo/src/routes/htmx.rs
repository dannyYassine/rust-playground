use std::sync::Arc;

use axum::{Json, Router, extract::Query, http::StatusCode};

use crate::AppState;
use crate::models::User;
use crate::requests::{CreateUserRequest, GetUserRequest};
use crate::services::Route;
use crate::templates::UserTemplate;

pub fn router() -> Router<Arc<AppState>> {
    Route::new()
        .get(
            "/htmx/user",
            |Query(params): Query<GetUserRequest>| async move {
                let users: Vec<User> = vec![
                    User {
                        id: 1,
                        name: String::from("Danny"),
                        email: String::from("dan@dan.com"),
                        password: None,
                    },
                    User {
                        id: 2,
                        name: String::from("Dan"),
                        email: String::from("dan@dan2.com"),
                        password: None,
                    },
                ];

                let user = users
                    .iter()
                    .find(|user| {
                        if let Some(ref name) = params.name {
                            user.name == *name
                        } else {
                            false
                        }
                    })
                    .cloned();

                user.map(|u| UserTemplate {
                    name: u.name,
                    email: u.email,
                })
                .ok_or(StatusCode::NOT_FOUND)
            },
        )
        .post(
            "/htmx/user",
            |Json(payload): Json<CreateUserRequest>| async move {
                UserTemplate {
                    name: payload.name,
                    email: payload.email,
                }
            },
        )
        .into_router()
}
