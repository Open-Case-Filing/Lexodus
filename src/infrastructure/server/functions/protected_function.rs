use leptos::*;
use crate::infrastructure::di::Container;
use crate::infrastructure::server::middleware::auth::auth_middleware;

#[server(ProtectedFunction, "/api")]
pub async fn protected_function_handler(/* params */) -> Result<String, ServerFnError> {
    let db_url = spin_sdk::variables::get("db_url").expect("db_url must be set");
    let jwt_secret = spin_sdk::variables::get("jwt_secret").expect("jwt_secret must be set");
    let container = Container::new(&db_url, jwt_secret);
    let auth_service = container.auth_service();

    auth_middleware(&auth_service, leptos::use_context::<leptos::Request>().unwrap(), |req| {
        // Your protected function logic here
        // You can access the user_id from req.extensions() if needed
        Ok("Access granted to protected function".to_string())
    }).await
}
