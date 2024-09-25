// src/domain/repositories/case_repository.rs
use async_trait::async_trait;
use crate::domain::models::case::Case;

#[async_trait]
pub trait CaseRepository: Send + Sync {
    async fn create(&self, case: &Case) -> Result<i64, String>;
    async fn get(&self, id: i64) -> Result<Case, String>;
    async fn update(&self, case: &Case) -> Result<(), String>;
    async fn delete(&self, id: i64) -> Result<(), String>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Case>, String>;
}
