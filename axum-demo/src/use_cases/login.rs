use std::sync::Arc;

use component::Injectable;
use sqlx::PgPool;

use crate::{models::User, services::UserLoginService};

#[derive(Debug)]
pub enum LoginError {
    #[allow(dead_code)]
    InvalidEmail(String),
    InvalidPassword(String),
    #[allow(dead_code)]
    EmailNotFound(String),
    UserNotFound(String),
    DatabaseError(String),
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LoginError::InvalidEmail(msg) => msg,
            LoginError::InvalidPassword(msg) => msg,
            LoginError::EmailNotFound(msg) => msg,
            LoginError::UserNotFound(msg) => msg,
            LoginError::DatabaseError(msg) => msg,
        };
        write!(f, "{}", msg)
    }
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<User, LoginError>;
    async fn find_by_email(&self, email: &str) -> Result<User, LoginError>;
    #[allow(dead_code)]
    async fn save(&self, user: &User) -> Result<User, LoginError>;
    #[allow(dead_code)]
    async fn create(&self, name: &str, email: &str, password: &str) -> Result<User, LoginError>;
    #[allow(dead_code)]
    async fn delete(&self, user: &User) -> Result<bool, LoginError>;
}

#[derive(Injectable)]
pub struct UserRepo {
    #[inject(registered)]
    pool: Arc<PgPool>,
}

#[async_trait::async_trait]
impl UserRepository for UserRepo {
    async fn find_by_id(&self, id: i32) -> Result<User, LoginError> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password FROM users WHERE id = $1 and deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| LoginError::DatabaseError(e.to_string()))?
        .ok_or_else(|| {
            LoginError::UserNotFound("No account found with that email address.".to_string())
        })
    }
    async fn find_by_email(&self, email: &str) -> Result<User, LoginError> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password FROM users WHERE email = $1 and deleted_at IS NULL",
        )
        .bind(email)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| LoginError::DatabaseError(e.to_string()))?
        .ok_or_else(|| {
            LoginError::UserNotFound("No account found with that email address.".to_string())
        })
    }

    async fn save(&self, user: &User) -> Result<User, LoginError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)
             ON CONFLICT (email) DO UPDATE SET name = EXCLUDED.name, password = EXCLUDED.password
             RETURNING id, name, email, password",
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| LoginError::DatabaseError(e.to_string()))
    }

    async fn create(&self, name: &str, email: &str, password: &str) -> Result<User, LoginError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)
             RETURNING id, name, email, password",
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| LoginError::DatabaseError(e.to_string()))
    }

    async fn delete(&self, user: &User) -> Result<bool, LoginError> {
        let result = sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(&user.email)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| LoginError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(Injectable)]
pub struct LoginUseCase {
    pub user_login_service: Arc<UserLoginService>,
}

impl LoginUseCase {
    pub async fn execute(&self, email: &str, password: &str) -> anyhow::Result<User, LoginError> {
        let user = self
            .user_login_service
            .fetch_user_with_email_password(email, password)
            .await?;

        return Ok(user);
    }
}
