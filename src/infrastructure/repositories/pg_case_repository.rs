// src/infrastructure/repositories/pg_case_repository.rs

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use crate::domain::models::case::Case;
        use crate::domain::repositories::case_repository::CaseRepository;
        use anyhow::Result;
        use chrono::Utc;

        pub struct PgCaseRepository {
            conn: Connection,
        }

        impl PgCaseRepository {
            pub fn new(db_url: &str) -> Result<Self> {
                let conn = Connection::open(db_url)?;
                Ok(Self { conn })
            }

            pub fn generate_case_number(&self, district_code: &str, case_type: &str) -> Result<String> {
                let year = Utc::now().year();
                let sql = "INSERT INTO case_number_sequences (district_code, year, current_value)
                           VALUES ($1, $2, 1)
                           ON CONFLICT (district_code, year)
                           DO UPDATE SET current_value = case_number_sequences.current_value + 1
                           RETURNING current_value";
                let result = self.conn.query(sql, &[ParameterValue::Str(district_code.to_string()), ParameterValue::Int32(year)])?;
                if let Some(row) = result.get(0) {
                    let sequence: i32 = row.get(0).parse()?;
                    Ok(format!("{}:{}-{:05}-{}", district_code, year % 100, sequence, case_type))
                } else {
                    Err(anyhow::anyhow!("Failed to generate case number"))
                }
            }
        }


      #[async_trait::async_trait]
      impl CaseRepository for PgCaseRepository {
          async fn save(&self, case: &Case) -> Result<i64> {
              let sql = "INSERT INTO cases (case_number, title, status, filed_date, court_id, current_court_id, judge_id, district_code, is_mdl, mdl_status)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                         RETURNING id";

              let result = self.conn.query(sql, &[
                  ParameterValue::Str(case.case_number.clone()),
                  ParameterValue::Str(case.title.clone()),
                  ParameterValue::Str(case.status.clone()),
                  ParameterValue::Str(case.filed_date.clone()),
                  ParameterValue::Int64(case.court_id),
                  ParameterValue::Int64(case.current_court_id),
                  ParameterValue::Int64(case.judge_id.unwrap_or(-1)),
                  ParameterValue::Str(case.district_code.clone()),
                  ParameterValue::Bool(case.is_mdl),
                  ParameterValue::Str(case.mdl_status.clone().unwrap_or_default()),
              ])?;

              if let Some(row) = result.get(0) {
                  Ok(row.get(0).parse()?)
              } else {
                  Err(anyhow::anyhow!("Failed to insert case"))
              }
          }

          async fn find_by_id(&self, id: i64) -> Result<Option<Case>> {
              let sql = "SELECT * FROM cases WHERE id = $1";
              let result = self.conn.query(sql, &[ParameterValue::Int64(id)])?;

              if let Some(row) = result.get(0) {
                  Ok(Some(Case {
                      id: row.get(0).parse()?,
                      case_number: row.get(1),
                      title: row.get(2),
                      status: row.get(3),
                      filed_date: row.get(4),
                      closed_date: row.get(5),
                      court_id: row.get(6).parse()?,
                      current_court_id: row.get(7).parse()?,
                      judge_id: row.get(8).parse().ok(),
                      is_mdl: row.get(9).parse()?,
                      mdl_status: Some(row.get(10)),
                      district_code: row.get(11),
                  }))
              } else {
                  Ok(None)
              }
          }

          // Implement other repository methods as needed...
      }


    }
}
