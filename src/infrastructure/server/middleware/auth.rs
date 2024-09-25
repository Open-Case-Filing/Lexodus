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
