// src/domain/repositories/mdl_repository.rs

use async_trait::async_trait;
use anyhow::Result;
use crate::domain::models::MDLProceeding;

#[async_trait]
pub trait MDLRepository {
    async fn save(&self, mdl: &MDLProceeding) -> Result<i64>;
    async fn find_by_id(&self, id: i64) -> Result<Option<MDLProceeding>>;
    // fn generate_mdl_number(&self) -> Result<String>;
    // Removed this in favor of using it from services/generate_mdl_number.rs
}
