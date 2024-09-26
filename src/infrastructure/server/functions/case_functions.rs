// src/infrastructure/server/functions/case_functions.rs
use leptos::*;
use crate::domain::models::new_case::NewCase;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::infrastructure::di::container::Container;
        use spin_sdk::variables;
    }
}

#[server(CreateCase, "/api")]
pub async fn create_case_handler(new_case: NewCase, user_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").expect("db_url must be set");
    let container = Container::new(&db_url);
    let case_service = container.case_service();
    match case_service.create_case(new_case, user_id).await {
        Ok(case) => Ok(format!("Case created with number: {}", case.case_number)),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(GenerateCaseNumber, "/api")]
pub async fn generate_case_number_handler(district_code: String, case_type: String) -> Result<String, ServerFnError> {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let db_url = variables::get("db_url").expect("db_url must be set");
            let container = Container::new(&db_url);
            let case_service = container.case_service();
            case_service.generate_case_number(&district_code, &case_type)
                .map_err(|e| ServerFnError::ServerError(e.to_string()))
        } else {
            Err(ServerFnError::ServerError("Server-side rendering is not enabled".into()))
        }
    }
}
