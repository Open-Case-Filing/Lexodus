use crate::domain::models::user::User;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use argon2::{self, Config};
use server_fn::ServerFnError;

pub struct UserService<R: PostgresUserRepository> {
    repository: R,
}

impl<R: PostgresUserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, ServerFnError> {
        let salt = b"somesalt"; // In production, generate a proper salt
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt, &config)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    pub async fn create_user(
        &self,
        username: String,
        password_hash: String,
        role_id: i64,
    ) -> Result<User, ServerFnError> {
        self.repository.create_user(username, password_hash, role_id).await
    }
}
