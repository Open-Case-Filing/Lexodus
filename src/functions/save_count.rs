use leptos::*;


#[server(SaveCount, "/api")]
pub async fn save_count(count: u32) -> Result<(), ServerFnError<String>> {
    println!("Saving value {count}");
    let store = spin_sdk::key_value::Store::open_default().map_err(|e| e.to_string())?;
    store.set_json("lexodus", &count).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}