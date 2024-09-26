// src/server/middleware/rbac.rs
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
use crate::domain::models::user::UserRole;

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
}}
