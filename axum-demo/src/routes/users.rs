use std::sync::Arc;

use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};

use crate::AppState;
use crate::models::User;
use crate::requests::{CreateUserRequest, GetUserRequest};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/users", get(list).post(create))
}

async fn list(Query(params): Query<GetUserRequest>) -> Result<Json<User>, StatusCode> {
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
        .find(|&user| {
            if let Some(name) = params.name.as_ref() {
                user.name == *name
            } else {
                false
            }
        })
        .cloned();

    user.ok_or(StatusCode::NOT_FOUND).map(|u| Json(u))

    // match user {
    //     Some(u) => Ok(Json(u)),
    //     None => Err(StatusCode::NOT_FOUND),
    // }
}

async fn create(Json(payload): Json<CreateUserRequest>) -> Json<User> {
    Json(User {
        id: 0,
        name: payload.name,
        email: payload.email,
        password: None,
    })
}
