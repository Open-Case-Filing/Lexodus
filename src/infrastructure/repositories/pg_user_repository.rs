// src/infrastructure/repositories/pg_user_activity_repository.rs
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
use async_trait::async_trait;
use crate::domain::models::user_activity::UserActivity;
use crate::domain::repositories::UserActivityRepository;
use spin_sdk::pg::{Connection, ParameterValue};
use std::sync::Arc;

pub struct PgUserActivityRepository {
    conn: Arc<Connection>,
}

impl PgUserActivityRepository {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserActivityRepository for PgUserActivityRepository {
    async fn log(&self, activity: &UserActivity) -> Result<(), String> {
        let sql = "INSERT INTO user_activities (user_id, action_type, action_details, timestamp)
                   VALUES ($1, $2, $3, $4)";
        self.conn.execute(sql, &[
            ParameterValue::Int64(activity.user_id),
            ParameterValue::Str(&activity.action_type),
            ParameterValue::Str(&activity.action_details),
            ParameterValue::Str(&activity.timestamp.to_rfc3339()),
        ]).map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_user_activities(&self, user_id: i64, limit: i64) -> Result<Vec<UserActivity>, String> {
        let sql = "SELECT * FROM user_activities WHERE user_id = $1 ORDER BY timestamp DESC LIMIT $2";
        let rows = self.conn.query(sql, &[ParameterValue::Int64(user_id), ParameterValue::Int64(limit)])
            .map_err(|e| e.to_string())?;

        // Convert rows to Vec<UserActivity>
        // You'll need to implement this conversion based on your database schema
        // and UserActivity struct definition
        unimplemented!("Convert database rows to Vec<UserActivity>")
    }
}


    }
}
