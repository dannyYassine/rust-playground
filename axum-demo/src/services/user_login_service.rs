use std::sync::Arc;

use component::Injectable;

use crate::{
    models::User,
    services::{NewFromContainer, ServiceRegistry},
    use_cases::{LoginError, UserRepo, UserRepository},
};

#[derive(Injectable)]
pub struct UserLoginService {
    pub user_repo: Arc<dyn UserRepository>,
}

impl NewFromContainer for UserLoginService {
    fn new_from_container(registry: &ServiceRegistry) -> Self {
        Self {
            user_repo: registry.get_or_new::<UserRepo>(),
        }
    }
}

impl UserLoginService {
    pub async fn fetch_user_with_email_password(
        &self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<User, LoginError> {
        let user = self.user_repo.find_by_email(email).await?;

        if !user.has_same_password(password) {
            return Err(LoginError::InvalidPassword(
                "Incorrect password. Please try again.".to_string(),
            ));
        }

        return Ok(user);
    }
}
