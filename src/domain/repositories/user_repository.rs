// src/domain/repositories/user_repository.rs

use async_trait::async_trait;
use anyhow::Result;
use crate::domain::models::User;  // Assuming you have a User model

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn create(&self, user: &User) -> Result<User>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn delete(&self, id: i64) -> Result<()>;
    async fn list(&self, limit: i32, offset: i32) -> Result<Vec<User>>;
}


use crate::domain::models::user_activity::UserActivity;

#[async_trait]
pub trait UserActivityRepository: Send + Sync {
    async fn log(&self, activity: &UserActivity) -> Result<(), String>;
    async fn get_user_activities(&self, user_id: i64, limit: i64) -> Result<Vec<UserActivity>, String>;
}
