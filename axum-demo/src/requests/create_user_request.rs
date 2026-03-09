use serde::Deserialize;

#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}
