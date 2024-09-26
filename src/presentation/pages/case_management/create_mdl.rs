// src/presentation/pages/case_management/create_mdl.rs

use leptos::*;
use leptos_router::ActionForm;
use crate::infrastructure::server::functions::mdl::CreateMDLProceeding;

#[component]
pub fn CreateMDLPage() -> impl IntoView {
    let create_mdl = create_server_action::<CreateMDLProceeding>();

    view! {
        <h1>"Create MDL Proceeding"</h1>
        <ActionForm action=create_mdl>
            <label>
                "MDL Title"
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Create MDL"/>
        </ActionForm>

        {move || create_mdl.value().get().map(|result| {
            match result {
                Ok(message) => view! { <p>"Success: " {message}</p> },
                Err(e) => view! { <p>"Error: " {e.to_string()}</p> },
            }
        })}
    }
}
