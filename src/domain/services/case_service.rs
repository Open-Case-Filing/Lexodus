// src/domain/services/case_service.rs or wherever you need to generate a case number

use crate::infrastructure::repositories::pg_case_repository::PgCaseRepository;
use anyhow::Result;

pub struct CaseService {
    case_repo: PgCaseRepository,
}

impl CaseService {
    pub fn new(case_repo: PgCaseRepository) -> Self {
        Self { case_repo }
    }

    pub fn generate_case_number(&self, district_code: &str, case_type: &str) -> Result<String> {
        self.case_repo.generate_case_number(district_code, case_type)
    }

    // Other service methods...
}
