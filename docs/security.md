1. Authentication Middleware

Implement authentication middleware that checks for a valid token or session before allowing access to protected endpoints.

```rust
// src/server/middleware/auth.rs
use leptos::*;
use crate::domain::services::auth_service::AuthService;

pub async fn auth_middleware<T>(
    auth_service: &AuthService,
    req: leptos::Request,
    next: impl Fn(leptos::Request) -> T,
) -> Result<T, ServerFnError> {
    let token = req.headers().get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(ServerFnError::ServerError("No token provided".into()))?;

    match auth_service.verify_token(token).await {
        Ok(user_id) => {
            // Add the user_id to the request context
            req.extensions().insert(user_id);
            Ok(next(req))
        },
        Err(_) => Err(ServerFnError::ServerError("Invalid token".into())),
    }
}
```

2. Role-Based Access Control (RBAC)

Implement RBAC to restrict access based on user roles.

```rust
// src/domain/models/user.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: UserRole,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Judge,
    Attorney,
    Clerk,
}

// src/server/middleware/rbac.rs
pub fn require_role(required_role: UserRole) -> impl Fn(leptos::Request) -> Result<leptos::Request, ServerFnError> {
    move |req: leptos::Request| {
        let user = req.extensions().get::<User>()
            .ok_or(ServerFnError::ServerError("User not authenticated".into()))?;

        if user.role == required_role {
            Ok(req)
        } else {
            Err(ServerFnError::ServerError("Insufficient permissions".into()))
        }
    }
}
```

3. Apply Middleware to Server Functions

Modify your server functions to use these middleware:

```rust
// src/server/functions/case_functions.rs
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
    let user_id = use_context::<i64>().ok_or(ServerFnError::ServerError("User not authenticated".into()))?;

    case_service.create_case(user_id, case)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
```

4. HTTPS

Ensure all API communications are over HTTPS to encrypt data in transit. This is typically configured at the server level (e.g., in your reverse proxy or cloud platform settings).

5. Rate Limiting

Implement rate limiting to prevent abuse of your API:

```rust
// src/server/middleware/rate_limit.rs
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use leptos::*;

struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: HashMap::new(),
            max_requests,
            window,
        }
    }

    fn is_rate_limited(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let requests = self.requests.entry(key.to_string()).or_insert_with(Vec::new);

        requests.retain(|&time| now.duration_since(time) < self.window);

        if requests.len() >= self.max_requests {
            true
        } else {
            requests.push(now);
            false
        }
    }
}

pub fn rate_limit(max_requests: usize, window: Duration) -> impl Fn(leptos::Request) -> Result<leptos::Request, ServerFnError> {
    let limiter = Mutex::new(RateLimiter::new(max_requests, window));

    move |req: leptos::Request| {
        let ip = req.headers().get("X-Forwarded-For")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("unknown");

        let mut limiter = limiter.lock().unwrap();
        if limiter.is_rate_limited(ip) {
            Err(ServerFnError::ServerError("Rate limit exceeded".into()))
        } else {
            Ok(req)
        }
    }
}
```

6. Input Validation

Implement thorough input validation for all API endpoints to prevent injection attacks and other security issues. This can be done at the domain model level, as shown earlier.

7. Error Handling

Implement proper error handling to avoid leaking sensitive information:

```rust
// src/server/errors.rs
use leptos::*;

pub fn handle_error(error: ServerFnError) -> ServerResponse {
    match error {
        ServerFnError::ServerError(msg) if msg.contains("Rate limit exceeded") => {
            ServerResponse::new(StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded")
        },
        ServerFnError::ServerError(msg) if msg.contains("Invalid token") => {
            ServerResponse::new(StatusCode::UNAUTHORIZED, "Invalid token")
        },
        ServerFnError::ServerError(msg) if msg.contains("Insufficient permissions") => {
            ServerResponse::new(StatusCode::FORBIDDEN, "Insufficient permissions")
        },
        _ => ServerResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
    }
}
```

8. Logging and Monitoring

Implement comprehensive logging and monitoring to detect and respond to potential security threats:

```rust
// src/infrastructure/logging/security_logger.rs
use log::{info, warn, error};

pub struct SecurityLogger;

impl SecurityLogger {
    pub fn log_access(user_id: i64, endpoint: &str, ip: &str) {
        info!("Access: user_id={}, endpoint={}, ip={}", user_id, endpoint, ip);
    }

    pub fn log_failed_auth(ip: &str, reason: &str) {
        warn!("Failed auth attempt: ip={}, reason={}", ip, reason);
    }

    pub fn log_security_event(event_type: &str, details: &str) {
        error!("Security event: type={}, details={}", event_type, details);
    }
}
```

Use this logger in your middleware and server functions.

9. Cross-Origin Resource Sharing (CORS)

Configure CORS to restrict which domains can access your API:

```rust
// src/server/middleware/cors.rs
use leptos::*;

pub fn cors_middleware(allowed_origins: Vec<String>) -> impl Fn(leptos::Request) -> Result<leptos::Request, ServerFnError> {
    move |mut req: leptos::Request| {
        let origin = req.headers().get("Origin")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        if allowed_origins.contains(&origin.to_string()) {
            req.headers_mut().insert("Access-Control-Allow-Origin", origin.parse().unwrap());
            req.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE".parse().unwrap());
            req.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());
            Ok(req)
        } else {
            Err(ServerFnError::ServerError("CORS error: Origin not allowed".into()))
        }
    }
}
```

By implementing these security measures, you can significantly enhance the security of your API endpoints. Remember to regularly update dependencies, perform security audits, and stay informed about new security best practices and potential vulnerabilities.
