use leptos::*;
use leptos_router::*;
use serde::{Serialize, Deserialize};


use crate::layouts::default::DefaultLayout;
#[derive(Params, PartialEq, Clone)]
struct CaseParams {
    id: Option<String>,
}

#[component]
pub fn ViewCase() -> impl IntoView {
    let params = use_params::<CaseParams>();
    let case_id = move || params.with(|p| p.as_ref().ok().and_then(|p| p.id.clone()).unwrap_or_default());

    let case_details = create_resource(
        case_id,
        move |id| async move {
            match get_case_details(id).await {
                Ok(details) => Ok(details),
                Err(err) => {
                    log::error!("Error fetching case details: {:?}", err);
                    Err(err)
                }
            }
        }
    );

    view! {
        <DefaultLayout>
            <h2>"View Case"</h2>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || case_details.get().map(|result| match result {
                    Ok(case) => view! {
                        <div>
                            <p>"Case ID: "{&case.id}</p>
                            <p>"Case Number: "{&case.number}</p>
                            <p>"Title: "{&case.title}</p>
                            <p>"Status: "{&case.status}</p>
                        </div>
                    },
                    Err(e) => view! {
                        <div>
                            <p>"Error loading case details: "{e.to_string()}</p>
                            <p>"Please check the server logs for more information."</p>
                        </div>
                    },
                })}
            </Suspense>
        </DefaultLayout>
    }
}

#[server(GetCaseDetails, "/api")]
pub async fn get_case_details(case_id: String) -> Result<CaseDetails, ServerFnError> {
    // For debugging purposes, let's log the incoming case_id
    log::info!("Fetching details for case ID: {}", case_id);

    // Implement the logic to fetch case details from the database
    // For now, we'll return mock data
    let details = CaseDetails {
        id: case_id,
        number: "2023-CV-001".to_string(),
        title: "Sample Case".to_string(),
        status: "Open".to_string(),
    };

    // Log the details we're about to return
    log::info!("Returning case details: {:?}", details);

    Ok(details)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDetails {
    id: String,
    number: String,
    title: String,
    status: String,
}
