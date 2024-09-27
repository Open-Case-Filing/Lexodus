use crate::domain::models::user::User;
use async_trait::async_trait;
use server_fn::ServerFnError;
use spin_sdk::pg::{Connection, ParameterValue};

#[async_trait]
pub trait PostgresUserRepository {
    async fn create_user(
        &self,
        username: String,
        password_hash: String,
        role_id: i64,
    ) -> Result<User, ServerFnError>;
}

pub struct PostgresUserRepo {
    db_url: String,
}

impl PostgresUserRepo {
    pub fn new(db_url: String) -> Self {
        Self { db_url }
    }

    async fn get_connection(&self) -> Result<Connection, ServerFnError> {
        Connection::open(&self.db_url)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}

#[async_trait]
impl PostgresUserRepository for PostgresUserRepo {
    async fn create_user(
        &self,
        username: String,
        password_hash: String,
        role_id: i64,
    ) -> Result<User, ServerFnError> {
        let conn = self.get_connection().await?;

        let sql = "INSERT INTO users (username, password_hash, role_id)
                   VALUES ($1, $2, $3) RETURNING id, username, password_hash, role_id";

        let row = conn.query_one(
            sql,
            &[
                ParameterValue::Str(username.clone()),
                ParameterValue::Str(password_hash.clone()),
                ParameterValue::Int64(role_id),
            ],
        ).map_err(|e| ServerFnError::ServerError(e.to_string()))?;

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            role_id: row.get("role_id"),
        };

        Ok(user)
    }
}
