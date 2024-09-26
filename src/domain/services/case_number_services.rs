

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
      use chrono::Utc;
      use crate::infrastructure::repositories::pg_case_repository::PgCaseRepository;


pub fn generate_case_number(district_code: &str, case_type: &str) -> Result<String, anyhow::Error> {
    let year = Utc::now().year();
    let sequence = PgCaseRepository::get_next_sequence(district_code, year)?;
    Ok(format!("{}:{}-{:05}-{}", district_code, year % 100, sequence, case_type))
}


    }
}
