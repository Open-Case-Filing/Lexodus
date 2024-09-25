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
