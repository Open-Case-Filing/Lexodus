// src/application/commands/create_case.rs
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
      use anyhow::Result;
      use spin_sdk::variables;
      use crate::domain::models::Case;
      use crate::domain::repositories::CaseRepository;
      use crate::infrastructure::repositories::pg_case_repository::PgCaseRepository;
    }
}


#[cfg(feature = "ssr")]
pub async fn create_case(
    title: String,
    district_code: String,
    court_id: i64,
    judge_id: Option<i64>,
) -> Result<i64> {
    let db_url = variables::get("db_url").expect("db_url must be set");
    let repo = PgCaseRepository::new(&db_url)?;

    let case_number = repo.generate_case_number(&district_code, "CV")?;
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

    repo.save(&case).await
}
