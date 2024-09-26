// src/infrastructure/di/container.rs

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
use crate::infrastructure::repositories::{pg_case_repository::PgCaseRepository, pg_user_repository::PgUserActivityRepository};

use crate::domain::services::case_service::CaseService;

use spin_sdk::pg::Connection;

pub struct Container {
    db_connection: Connection,
}

impl Container {
    pub fn new(db_url: &str) -> Self {
        let db_connection = Connection::open(db_url).expect("Failed to connect to database");
        Self { db_connection }
    }

    pub fn case_service(&self) -> CaseService<PgCaseRepository, PgUserRepository> {
        let case_repo = PgCaseRepository::new(self.db_connection.clone());
        let user_repo = PgUserRepository::new(self.db_connection.clone());
        CaseService::new(case_repo, user_repo)
    }

    pub fn auth_service(&self) -> AuthService<PgUserRepository> {
        let user_repo = PgUserRepository::new(self.db_connection.clone());
        AuthService::new(user_repo, self.jwt_secret.clone())
    }

    // Other service getters...
}
}}
