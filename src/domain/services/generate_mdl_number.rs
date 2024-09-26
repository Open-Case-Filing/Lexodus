use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

      use crate::infrastructure::repositories::pg_case_repository::PgCaseRepository;
      use anyhow::Result;
      use chrono::Utc;

      pub fn generate() -> Result<String> {
          let db_url = spin_sdk::variables::get("db_url").expect("db_url must be set");
          let repo = PgCaseRepository::new(&db_url)?;

          let year = Utc::now().year();
          let sequence = repo.get_next_sequence("MDL", year)?;

          Ok(format!("MDL-{}-{:04}", year, sequence))
      }
}}
