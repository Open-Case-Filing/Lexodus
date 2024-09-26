use leptos::*;
#[cfg(feature = "ssr")]

#[cfg(feature = "ssr")]
use spin_sdk::variables;
#[cfg(feature = "ssr")]
use crate::infrastructure::repositories::pg_case_repository::PgCaseRepository;
#[cfg(feature = "ssr")]
use crate::domain::models::case::Case;  // Make sure this import is correct

use leptos::ServerFnError;

#[server]
pub async fn create_case(
    title: String,
    district_code: String,
    court_id: i64,
    judge_id: Option<i64>,
) -> Result<i64, ServerFnError> {
    let db_url = variables::get("db_url").map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let repo = PgCaseRepository::new(&db_url).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let case_number = repo.generate_case_number(&district_code, "CV").map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let filed_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let case = Case {
        id: 0, // This will be set by the database
        case_number,
        title,
        status: "OPEN".to_string(),
        filed_date,
        closed_date: None,
        court_id,
        current_court_id: court_id,
        judge_id,
        is_mdl: false,
        mdl_status: None,
        district_code,
    };
    repo.save(&case).await.map_err(|e| ServerFnError::ServerError(e.to_string()))
}
