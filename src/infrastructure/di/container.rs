
use std::sync::Arc;
use crate::domain::repositories::case_repository::CaseRepository;
use crate::infrastructure::repositories::pg_case_repository::PgCaseRepository;
use crate::application::services::case_service::CaseService;
use crate::infrastructure::logging::logger::Logger;
use crate::server::middleware::auth::AuthMiddleware;

pub struct Container {
    case_repository: Arc<dyn CaseRepository>,
    logger: Logger,
}

impl Container {
    pub fn new() -> Self {
        let logger = Logger::new();
        let case_repository = Arc::new(PgCaseRepository::new()) as Arc<dyn CaseRepository>;
        Self { case_repository, logger }
    }

    pub fn case_service(&self) -> CaseService {
        CaseService::new(
            Box::new(Arc::clone(&self.case_repository) as Box<dyn CaseRepository>),
            self.logger.clone(),
        )
    }


    pub fn auth_middleware(&self) -> Arc<AuthMiddleware> {
        Arc::clone(&self.auth_middleware)
    }

}
