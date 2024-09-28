use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
use async_session::{Session, Result, chrono::Utc, SessionStore};
use spin_sdk::pg::{Connection, ParameterValue};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct PostgresStore {
    connection: Arc<Connection>,
    table_name: String,
}

#[derive(Debug, Clone)]
pub struct SessionRow {
    id: String,
    expiry: Option<i64>,
    session: String,
}

impl PostgresStore {
    pub fn from_connection(postgres_connection: Arc<Connection>) -> Self {
        Self {
            connection: postgres_connection,
            table_name: "async_sessions".to_string(),
        }
    }

    pub fn from_connection_with_table_name(postgres_connection: Arc<Connection>, table_name: impl AsRef<str>) -> Self {
        Self {
            connection: postgres_connection,
            table_name: table_name.as_ref().to_string(),
        }
    }

    pub fn with_table_name(mut self, table_name: impl AsRef<str>) -> Self {
        let table_name = table_name.as_ref();
        if table_name.is_empty()
            || !table_name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            panic!(
                "table name must be [a-zA-Z0-9_-]+, but {} was not",
                table_name
            );
        }

        self.table_name = table_name.to_owned();
        self
    }

    pub async fn migrate(&self) -> Result<()> {
        info!("migrating sessions on `{}`", self.table_name);

        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY NOT NULL,
                expiry BIGINT NULL,
                session TEXT NOT NULL
            )
            "#,
            self.table_name
        );

        self.connection.execute(&query, &[]).await?;
        Ok(())
    }

    pub async fn cleanup(&self) -> Result<()> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE expiry < $1
            "#,
            self.table_name
        );

        self.connection.execute(&query, &[&ParameterValue::Int8(Utc::now().timestamp())]).await?;
        Ok(())
    }

    pub async fn count(&self) -> Result<i64> {
        let query = format!("SELECT COUNT(*) FROM {}", self.table_name);
        let result = self.connection.query(&query, &[]).await?;
        let count: i64 = result[0].get("count");
        Ok(count)
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let query = format!(
            r#"
            SELECT * FROM {}
            WHERE id = $1 AND (expiry IS NULL OR expiry > $2)
            "#,
            self.table_name
        );

        let result = self.connection.query(&query, &[
            &ParameterValue::Text(id.to_string()),
            &ParameterValue::Int8(Utc::now().timestamp())
        ]).await?;

        let session_row = result.get(0).map(|row| SessionRow {
            id: row.get("id"),
            expiry: row.get("expiry"),
            session: row.get("session"),
        });

        let session: Option<String> = session_row.map(|s| s.session);

        Ok(session
            .map(|session| serde_json::from_str(&session))
            .transpose()?)
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let string = serde_json::to_string(&session)?;
        let expiry = session.expiry().map(|expiry| expiry.timestamp());

        let query = format!(
            r#"
            INSERT INTO {} (id, session, expiry)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
                expiry = EXCLUDED.expiry,
                session = EXCLUDED.session
            "#,
            self.table_name
        );

        self.connection.execute(&query, &[
            &ParameterValue::Text(id.to_string()),
            &ParameterValue::Text(string),
            &expiry.map_or(ParameterValue::Null, |e| ParameterValue::Int8(e)),
        ]).await?;

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let id = session.id();
        let query = format!(
            r#"
            DELETE FROM {} WHERE id = $1
            "#,
            self.table_name
        );

        self.connection.execute(&query, &[&ParameterValue::Text(id.to_string())]).await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result {
        let query = format!("DELETE FROM {}", self.table_name);
        self.connection.execute(&query, &[]).await?;
        Ok(())
    }
}
}}
