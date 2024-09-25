use leptos::*;
// src/infrastructure/server/functions/case_functions.rs

use crate::application::commands::create_case;

#[server(CreateCase, "/api")]
pub async fn create_case_handler(
    title: String,
    district_code: String,
    court_id: i64,
    judge_id: Option<i64>,
) -> Result<String, ServerFnError> {
    match create_case(title, district_code, court_id, judge_id).await {
        Ok(id) => Ok(format!("Case created with ID: {}", id)),
        Err(e) => Err(ServerFnError::ServerError(format!("Failed to create case: {}", e))),
    }
}
