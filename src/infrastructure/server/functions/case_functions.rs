use leptos::*;
use crate::infrastructure::di::Container;
use crate::domain::models::case::Case;
use crate::server::middleware::{auth_middleware, rbac};
use crate::domain::models::user::UserRole;

#[server(CreateCase, "/api", middleware = [auth_middleware, rbac::require_role(UserRole::Attorney)])]
pub async fn create_case(case: Case) -> Result<i64, ServerFnError> {
    let container = Container::new();
    let case_service = container.case_service();

    // Get the authenticated user_id from the request context
    let user_id = use_context::<i64>().ouse leptos::*;
    use crate::server::middleware::auth::auth_middleware;
    use crate::domain::models::case::Case;
    use crate::application::services::CaseService;
    use crate::infrastructure::di::Container;

    #[server(CreateCase, "/api")]
    pub async fn create_case(case: Case) -> Result<i64, ServerFnError> {
        // Apply auth middleware
        let req = use_context::<leptos::Request>()
            .ok_or_else(|| ServerFnError::ServerError("Request context not found".into()))?;
        let req = auth_middleware(req).await?;

        // Get user_id from the authenticated request
        let user_id = req.extensions()
            .get::<String>()
            .ok_or_else(|| ServerFnError::ServerError("User not authenticated".into()))?;

        // Use the container to get the CaseService
        let container = Container::new();
        let case_service = container.case_service();

        // Call the service method to create the case
        case_service.create_case(user_id.parse().unwrap(), case)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
k_or(ServerFnError::ServerError("User not authenticated".into()))?;

    case_service.create_case(user_id, case)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
