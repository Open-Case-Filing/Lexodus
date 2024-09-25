use crate::application::commands::create_case::{CreateCaseCommand, handle_create_case};
use crate::domain::models::case::Case;

use leptos::*;

#[server(CreateCase, "/api")]
pub async fn case_service(case: Case) -> Result<i64, ServerFnError> {
    use crate::infrastructure::di::container::Container;
    let container = Container::new();
    let case_service = container.case_service();

    case_service.create_case(case)
        .await
        .map_err(|e| ServerFnError::ServerError(e))
}
