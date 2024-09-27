use crate::domain::models::user::User;
use crate::application::services::user_service::UserService;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use server_fn::ServerFnError;
use std::sync::Arc;

/// Command for creating a user
pub struct CreateUserCommand {
    pub username: String,
    pub password: String,
    pub role_id: i64,
}

/// Handler for processing the `CreateUserCommand`
pub struct CreateUserHandler {
    pub user_service: Arc<UserService<PostgresUserRepository>>,
}

impl CreateUserHandler {
    /// Executes the command to create a user
    pub async fn handle(&self, command: CreateUserCommand) -> Result<User, ServerFnError> {
        if command.username.trim().is_empty() {
            return Err(ServerFnError::ServerError("Username cannot be empty".to_string()));
        }

        let hashed_password = self.user_service.hash_password(&command.password)?;

        let user = self.user_service.create_user(
            command.username,
            hashed_password,
            command.role_id,
        ).await?;

        Ok(user)
    }
}
