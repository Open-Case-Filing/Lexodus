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
