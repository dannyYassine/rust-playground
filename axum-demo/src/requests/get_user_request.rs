use serde::Deserialize;

#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct GetUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}
