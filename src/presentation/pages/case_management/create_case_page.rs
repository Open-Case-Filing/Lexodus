use leptos::*;
use leptos_router::ActionForm;
use crate::infrastructure::server::functions::case_functions::CreateCase;

#[component]
pub fn CreateCasePage() -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();

    view! {
        <h1>"Create New Case"</h1>
        <ActionForm action=create_case>
            <label>
                "Case Title"
                <input type="text" name="title"/>
            </label>
            <label>
                "District Code"
                <input type="text" name="district_code"/>
            </label>
            <input type="submit" value="Create Case"/>
        </ActionForm>

        // {move || create_case.value().map(|result| {
        //     match result {
        //         Ok(message) => view! { <p>"Success: " {message}</p> },
        //         Err(e) => view! { <p>"Error: " {e.to_string()}</p> },
        //     }
        // })}
    }
}
