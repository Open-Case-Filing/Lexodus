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
