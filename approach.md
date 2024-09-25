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

This approach allows you to maintain a clean separation of concerns while ensuring that related operations (like logging user activity) are performed consistently across your application. It also makes it easier to add new features or modify existing ones without changing the server function interfaces.

Can you go more in depth?

Certainly! I'll dive deeper into the architecture and explain each part in more detail, along with some additional considerations and best practices.

1. Domain Layer

First, let's define our domain models more comprehensively:

```rust
// src/domain/models/case.rs
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    pub id: Option<i64>,
    pub case_number: String,
    pub title: String,
    pub status: CaseStatus,
    pub filed_date: DateTime<Utc>,
    pub closed_date: Option<DateTime<Utc>>,
    pub court_id: i64,
    pub current_court_id: i64,
    pub judge_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CaseStatus {
    Open,
    Closed,
    Pending,
}

// src/domain/models/user_activity.rs
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Option<i64>,
    pub user_id: i64,
    pub action_type: String,
    pub action_details: String,
    pub timestamp: DateTime<Utc>,
}
```

2. Repository Traits

Define repository traits for each domain model:

```rust
// src/domain/repositories/case_repository.rs
use async_trait::async_trait;
use crate::domain::models::case::Case;

#[async_trait]
pub trait CaseRepository: Send + Sync {
    async fn create(&self, case: &Case) -> Result<i64, String>;
    async fn get(&self, id: i64) -> Result<Case, String>;
    async fn update(&self, case: &Case) -> Result<(), String>;
    async fn delete(&self, id: i64) -> Result<(), String>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Case>, String>;
    async fn search(&self, query: &str) -> Result<Vec<Case>, String>;
}

// src/domain/repositories/user_activity_repository.rs
use async_trait::async_trait;
use crate::domain::models::user_activity::UserActivity;

#[async_trait]
pub trait UserActivityRepository: Send + Sync {
    async fn log(&self, activity: &UserActivity) -> Result<(), String>;
    async fn get_user_activities(&self, user_id: i64, limit: i64) -> Result<Vec<UserActivity>, String>;
}
```

3. Service Layer

Create a service layer that uses these repositories:

```rust
// src/application/services/case_service.rs
use crate::domain::models::case::Case;
use crate::domain::models::user_activity::UserActivity;
use crate::domain::repositories::{CaseRepository, UserActivityRepository};
use chrono::Utc;

pub struct CaseService {
    case_repo: Box<dyn CaseRepository>,
    user_activity_repo: Box<dyn UserActivityRepository>,
}

impl CaseService {
    pub fn new(
        case_repo: Box<dyn CaseRepository>,
        user_activity_repo: Box<dyn UserActivityRepository>,
    ) -> Self {
        Self {
            case_repo,
            user_activity_repo,
        }
    }

    pub async fn create_case(&self, user_id: i64, mut case: Case) -> Result<i64, String> {
        // Ensure the case doesn't have an ID set
        case.id = None;

        // Create the case
        let case_id = self.case_repo.create(&case).await?;

        // Log the user activity
        let activity = UserActivity {
            id: None,
            user_id,
            action_type: "CREATE_CASE".to_string(),
            action_details: format!("Created case with ID: {}", case_id),
            timestamp: Utc::now(),
        };
        self.user_activity_repo.log(&activity).await?;

        Ok(case_id)
    }

    pub async fn get_case(&self, id: i64) -> Result<Case, String> {
        self.case_repo.get(id).await
    }

    pub async fn update_case(&self, user_id: i64, case: Case) -> Result<(), String> {
        // Ensure the case has an ID
        if case.id.is_none() {
            return Err("Case ID is required for update".to_string());
        }

        // Update the case
        self.case_repo.update(&case).await?;

        // Log the user activity
        let activity = UserActivity {
            id: None,
            user_id,
            action_type: "UPDATE_CASE".to_string(),
            action_details: format!("Updated case with ID: {}", case.id.unwrap()),
            timestamp: Utc::now(),
        };
        self.user_activity_repo.log(&activity).await?;

        Ok(())
    }

    // Add other methods as needed...
}
```

4. Repository Implementations

Implement the repository traits for your specific database (in this case, PostgreSQL):

```rust
// src/infrastructure/repositories/pg_case_repository.rs
use async_trait::async_trait;
use crate::domain::models::case::{Case, CaseStatus};
use crate::domain::repositories::CaseRepository;
use spin_sdk::pg::{Connection, ParameterValue};
use spin_sdk::variables;
use chrono::{DateTime, Utc};

pub struct PgCaseRepository;

#[async_trait]
impl CaseRepository for PgCaseRepository {
    async fn create(&self, case: &Case) -> Result<i64, String> {
        let db_url = variables::get("db_url").unwrap();
        let conn = Connection::open(&db_url).map_err(|e| e.to_string())?;

        let sql = "INSERT INTO cases (case_number, title, status, filed_date, court_id, current_court_id, judge_id)
                   VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id";
        let row = conn.query_row(sql, &[
            ParameterValue::Str(&case.case_number),
            ParameterValue::Str(&case.title),
            ParameterValue::Str(&case.status.to_string()),
            ParameterValue::Str(&case.filed_date.to_rfc3339()),
            ParameterValue::Int64(case.court_id),
            ParameterValue::Int64(case.current_court_id),
            case.judge_id.map(ParameterValue::Int64).unwrap_or(ParameterValue::DbNull),
        ]).map_err(|e| e.to_string())?;

        Ok(row[0].as_int64().unwrap_or_default())
    }

    async fn get(&self, id: i64) -> Result<Case, String> {
        let db_url = variables::get("db_url").unwrap();
        let conn = Connection::open(&db_url).map_err(|e| e.to_string())?;

        let sql = "SELECT id, case_number, title, status, filed_date, closed_date, court_id, current_court_id, judge_id
                   FROM cases WHERE id = $1";
        let row = conn.query_row(sql, &[ParameterValue::Int64(id)]).map_err(|e| e.to_string())?;

        Ok(Case {
            id: Some(row[0].as_int64().unwrap_or_default()),
            case_number: row[1].as_string().unwrap_or_default().to_string(),
            title: row[2].as_string().unwrap_or_default().to_string(),
            status: CaseStatus::from_str(row[3].as_string().unwrap_or_default()).unwrap_or(CaseStatus::Open),
            filed_date: DateTime::parse_from_rfc3339(row[4].as_string().unwrap_or_default())
                .map_err(|e| e.to_string())?
                .with_timezone(&Utc),
            closed_date: row[5].as_string().map(|s|
                DateTime::parse_from_rfc3339(s)
                    .map_err(|e| e.to_string())
                    .map(|dt| dt.with_timezone(&Utc))
            ).transpose()?,
            court_id: row[6].as_int64().unwrap_or_default(),
            current_court_id: row[7].as_int64().unwrap_or_default(),
            judge_id: row[8].as_int64(),
        })
    }

    // Implement other methods...
}

// src/infrastructure/repositories/pg_user_activity_repository.rs
// Similar implementation for UserActivityRepository...
```

5. Server Functions

Now, create thin wrappers around the service methods as server functions:

```rust
// src/server/functions/case_functions.rs
use leptos::*;
use crate::application::services::CaseService;
use crate::infrastructure::repositories::{PgCaseRepository, PgUserActivityRepository};
use crate::domain::models::case::Case;

#[server(CreateCase, "/api")]
pub async fn create_case(user_id: i64, case: Case) -> Result<i64, ServerFnError> {
    let case_repo = Box::new(PgCaseRepository);
    let user_activity_repo = Box::new(PgUserActivityRepository);
    let case_service = CaseService::new(case_repo, user_activity_repo);

    case_service.create_case(user_id, case)
        .await
        .map_err(|e| ServerFnError::ServerError(e))
}

#[server(GetCase, "/api")]
pub async fn get_case(id: i64) -> Result<Case, ServerFnError> {
    let case_repo = Box::new(PgCaseRepository);
    let user_activity_repo = Box::new(PgUserActivityRepository);
    let case_service = CaseService::new(case_repo, user_activity_repo);

    case_service.get_case(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e))
}

// Add other server functions as needed...
```

6. Dependency Injection

To make testing easier and allow for more flexibility, you can use a dependency injection container:

```rust
// src/infrastructure/di/container.rs
use std::sync::Arc;
use crate::domain::repositories::{CaseRepository, UserActivityRepository};
use crate::infrastructure::repositories::{PgCaseRepository, PgUserActivityRepository};
use crate::application::services::CaseService;

pub struct Container {
    case_repo: Arc<dyn CaseRepository>,
    user_activity_repo: Arc<dyn UserActivityRepository>,
}

impl Container {
    pub fn new() -> Self {
        let case_repo = Arc::new(PgCaseRepository) as Arc<dyn CaseRepository>;
        let user_activity_repo = Arc::new(PgUserActivityRepository) as Arc<dyn UserActivityRepository>;

        Self {
            case_repo,
            user_activity_repo,
        }
    }

    pub fn case_service(&self) -> CaseService {
        CaseService::new(
            Arc::clone(&self.case_repo) as Box<dyn CaseRepository>,
            Arc::clone(&self.user_activity_repo) as Box<dyn UserActivityRepository>,
        )
    }
}
```

Then, update your server functions to use the container:

```rust
// src/server/functions/case_functions.rs
use leptos::*;
use crate::infrastructure::di::Container;
use crate::domain::models::case::Case;

#[server(CreateCase, "/api")]
pub async fn create_case(user_id: i64, case: Case) -> Result<i64, ServerFnError> {
    let container = Container::new();
    let case_service = container.case_service();

    case_service.create_case(user_id, case)
        .await
        .map_err(|e| ServerFnError::ServerError(e))
}

// Update other server functions similarly...
```

7. Error Handling

Implement proper error handling throughout your application:

```rust
// src/domain/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    // Add other error types as needed
}

// Update your repositories and services to use this error type
```

8. Validation

Implement validation in your domain models:

```rust
// src/domain/models/case.rs
impl Case {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.case_number.is_empty() {
            errors.push("Case number cannot be empty".to_string());
        }

        if self.title.is_empty() {
            errors.push("Title cannot be empty".to_string());
        }

        // Add more validation rules

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Use validation in your service
impl CaseService {
    pub async fn create_case(&self, user_id: i64, case: Case) -> Result<i64, DomainError> {
        case.validate().map_err(|errors| DomainError::ValidationError(errors.join(", ")))?;
        // Rest of the method...
    }
}
```
