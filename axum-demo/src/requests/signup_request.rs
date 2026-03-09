use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
