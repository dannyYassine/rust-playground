use std::sync::Arc;

use component::Injectable;

use crate::{
    models::User,
    services::{NewFromContainer, ServiceRegistry},
    use_cases::{LoginError, UserRepo, UserRepository},
};

#[derive(Injectable)]
pub struct SignUpUseCase {
    pub user_repo: Arc<dyn UserRepository>,
}

impl NewFromContainer for SignUpUseCase {
    fn new_from_container(registry: &ServiceRegistry) -> Self {
        return Self {
            user_repo: registry.get_or_new::<UserRepo>(),
        };
    }
}

impl SignUpUseCase {
    pub async fn execute(
        &self,
        name: &str,
        email: &str,
        password: &str,
        confirm_password: &str,
    ) -> anyhow::Result<User, LoginError> {
        if password != confirm_password {
            return Err(LoginError::InvalidPassword(
                "Passwords do not match. Please try again.".to_string(),
            ));
        }

        let user = self.user_repo.find_by_email(email).await;

        if let Ok(u) = user
            && u.email == password
        {
            return Err(LoginError::InvalidEmail(
                "An account with that email already exists.".to_string(),
            ));
        }

        let new_user = self.user_repo.create(name, email, password).await?;

        return Ok(new_user);
    }
}
