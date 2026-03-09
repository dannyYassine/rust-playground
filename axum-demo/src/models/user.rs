use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

impl User {
    pub fn new(id: i32, name: String, email: String, password: Option<String>) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }

    pub fn has_same_password(&self, password: &str) -> bool {
        match &self.password {
            Some(p) => p == password,
            None => false,
        }
    }
}
