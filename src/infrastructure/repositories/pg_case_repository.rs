// src/infrastructure/repositories/pg_case_repository.rs
use async_trait::async_trait;
use crate::domain::models::case::Case;
use crate::domain::repositories::case_repository::CaseRepository;
use spin_sdk::pg::{Connection, ParameterValue};
use std::sync::Arc;

pub struct PgCaseRepository {
    conn: Arc<Connection>,
}

impl PgCaseRepository {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl CaseRepository for PgCaseRepository {
    async fn create(&self, case: &Case) -> Result<i64, String> {
        let sql = "INSERT INTO cases (case_number, title, status, filed_date, court_id, current_court_id, judge_id)
                   VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id";
        let row = self.conn.query_row(sql, &[
            ParameterValue::Str(&case.case_number),
            ParameterValue::Str(&case.title),
            ParameterValue::Str(&case.status.to_string()),
            ParameterValue::Str(&case.filed_date.to_rfc3339()),
            ParameterValue::Int64(case.court_id),
            ParameterValue::Int64(case.current_court_id),
            case.judge_id.map(ParameterValue::Int64).unwrap_or(ParameterValue::DbNull),
        ]).map_err(|e| e.to_string())?;

        Ok(row[0].as_int64().unwrap_or_default())
    }

    async fn get(&self, id: i64) -> Result<Case, String> {
        let sql = "SELECT * FROM cases WHERE id = $1";
        let row = self.conn.query_row(sql, &[ParameterValue::Int64(id)])
            .map_err(|e| e.to_string())?;

        // Convert row to Case struct
        // You'll need to implement this conversion based on your database schema
        // and Case struct definition
        unimplemented!("Convert database row to Case struct")
    }

    // Implement other methods...
}
