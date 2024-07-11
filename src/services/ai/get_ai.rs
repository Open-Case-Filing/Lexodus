use leptos::*;
// #[cfg(feature = "ssr")]
// use spin_sdk::http::{IntoResponse, Request, Response};
// #[cfg(feature = "ssr")]
// use spin_sdk::llm;

#[server(GetAI, "/api")]
pub async fn get_ai() -> Result<(), ServerFnError> {
    Ok(())
}
