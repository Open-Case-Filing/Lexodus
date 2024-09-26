1. Create a service layer that orchestrates multiple operations.
2. Keep the server functions as thin wrappers around these services.
3. Use dependency injection to allow for easier testing and flexibility.

Here's how we could restructure our code:

```rust
// src/server/services/case_service.rs
use crate::server::models::case::Case;
use crate::server::repositories::{CaseRepository, UserActivityRepository};

pub struct CaseService {
    case_repo: Box<dyn CaseRepository>,
    user_activity_repo: Box<dyn UserActivityRepository>,
}

impl CaseService {
    pub fn new(case_repo: Box<dyn CaseRepository>, user_activity_repo: Box<dyn UserActivityRepository>) -> Self {
        Self {
            case_repo,
            user_activity_repo,
        }
    }

    pub async fn create_case(&self, user_id: i64, case: Case) -> Result<i64, String> {
        // Create the case
        let case_id = self.case_repo.create(case).await?;

        // Log the user activity
        self.user_activity_repo.log_activity(user_id, "CREATE_CASE", &format!("Created case {}", case_id)).await?;

        Ok(case_id)
    }

    pub async fn get_case(&self, case_id: i64) -> Result<Case, String> {
        self.case_repo.get(case_id).await
    }

    // Add other case-related methods...
}

// src/server/repositories/case_repository.rs
use async_trait::async_trait;
use crate::server::models::case::Case;

#[async_trait]
pub trait CaseRepository: Send + Sync {
    async fn create(&self, case: Case) -> Result<i64, String>;
    async fn get(&self, id: i64) -> Result<Case, String>;
    // Add other methods...
}

// src/server/repositories/user_activity_repository.rs
use async_trait::async_trait;

#[async_trait]
pub trait UserActivityRepository: Send + Sync {
    async fn log_activity(&self, user_id: i64, action: &str, details: &str) -> Result<(), String>;
    // Add other methods...
}

// src/server/repositories/pg_case_repository.rs
use async_trait::async_trait;
use crate::server::models::case::Case;
use crate::server::repositories::CaseRepository;
use spin_sdk::pg::{Connection, ParameterValue};
use spin_sdk::variables;

pub struct PgCaseRepository;

#[async_trait]
impl CaseRepository for PgCaseRepository {
    async fn create(&self, case: Case) -> Result<i64, String> {
        let db_url = variables::get("db_url").unwrap();
        let conn = Connection::open(&db_url).map_err(|e| e.to_string())?;

        let sql = "INSERT INTO cases (case_number, title, status, filed_date) VALUES ($1, $2, $3, $4) RETURNING id";
        let row = conn.query_row(sql, &[
            ParameterValue::Str(case.case_number),
            ParameterValue::Str(case.title),
            ParameterValue::Str(case.status),
            ParameterValue::Str(case.filed_date),
        ]).map_err(|e| e.to_string())?;

        Ok(row[0].as_int64().unwrap_or_default())
    }

    async fn get(&self, id: i64) -> Result<Case, String> {
        // Implement get method...
        unimplemented!()
    }
}

// src/server/repositories/pg_user_activity_repository.rs
use async_trait::async_trait;
use crate::server::repositories::UserActivityRepository;
use spin_sdk::pg::{Connection, ParameterValue};
use spin_sdk::variables;

pub struct PgUserActivityRepository;

#[async_trait]
impl UserActivityRepository for PgUserActivityRepository {
    async fn log_activity(&self, user_id: i64, action: &str, details: &str) -> Result<(), String> {
        let db_url = variables::get("db_url").unwrap();
        let conn = Connection::open(&db_url).map_err(|e| e.to_string())?;

        let sql = "INSERT INTO user_activity_logs (user_id, action_type, action_details) VALUES ($1, $2, $3)";
        conn.execute(sql, &[
            ParameterValue::Int64(user_id),
            ParameterValue::Str(action),
            ParameterValue::Str(details),
        ]).map_err(|e| e.to_string())?;

        Ok(())
    }
}

// src/server/commands/case_management.rs
use leptos::*;
use crate::server::services::CaseService;
use crate::server::repositories::{PgCaseRepository, PgUserActivityRepository};
use crate::server::models::case::Case;

#[server(CreateCase, "/api")]
pub async fn create_case(user_id: i64, case: Case) -> Result<i64, ServerFnError> {
    let case_repo = Box::new(PgCaseRepository);
    let user_activity_repo = Box::new(PgUserActivityRepository);
    let case_service = CaseService::new(case_repo, user_activity_repo);

    case_service.create_case(user_id, case).await.map_err(|e| ServerFnError::ServerError(e))
}

// src/server/queries/case_management.rs
use leptos::*;
use crate::server::services::CaseService;
use crate::server::repositories::{PgCaseRepository, PgUserActivityRepository};
use crate::server::models::case::Case;

#[server(GetCase, "/api")]
pub async fn get_case(case_id: i64) -> Result<Case, ServerFnError> {
    let case_repo = Box::new(PgCaseRepository);
    let user_activity_repo = Box::new(PgUserActivityRepository);
    let case_service = CaseService::new(case_repo, user_activity_repo);

    case_service.get_case(case_id).await.map_err(|e| ServerFnError::ServerError(e))
}
```

This structure offers several benefits:

1. The `CaseService` orchestrates complex operations, such as creating a case and logging user activity.
2. Server functions (`create_case`, `get_case`) are thin wrappers around the service methods, making them easier to maintain and test.
3. Repositories abstract database operations, allowing for easier switching between different database implementations if needed.
4. Dependency injection (passing repositories to the service) allows for easier unit testing by mocking dependencies.

To use this in your Leptos components:

```rust
#[component]
pub fn CreateCaseForm() -> impl IntoView {
    let create_case_action = create_server_action::<CreateCase>();
    let user_id = use_context::<i64>().expect("User ID not found in context");

    view! {
        <ActionForm action=create_case_action>
            // ... form fields ...
            <button type="submit">"Create Case"</button>
        </ActionForm>
    }
}
```
