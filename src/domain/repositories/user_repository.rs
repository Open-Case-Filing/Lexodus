// src/domain/repositories/user_activity_repository.rs
use async_trait::async_trait;
use crate::domain::models::user_activity::UserActivity;

#[async_trait]
pub trait UserActivityRepository: Send + Sync {
    async fn log(&self, activity: &UserActivity) -> Result<(), String>;
    async fn get_user_activities(&self, user_id: i64, limit: i64) -> Result<Vec<UserActivity>, String>;
}
