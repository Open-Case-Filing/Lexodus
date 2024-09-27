use crate::application::commands::create_user::{CreateUserCommand, CreateUserHandler};
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepo;
use crate::application::services::user_service::UserService;
use crate::domain::models::user::User;
use std::sync::Arc;
use server_fn::{server, ServerFnError};
use spin_sdk::variables;

#[server(AddUser, "/api")]
pub async fn add_user(
    username: String,
    password: String,
    role_id: i64,
) -> Result<User, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let user_service = UserService::new(PostgresUserRepo::new(db_url));
    let handler = CreateUserHandler {
        user_service: Arc::new(user_service),
    };

    let command = CreateUserCommand {
        username,
        password,
        role_id,
    };

    handler.handle(command).await
}
