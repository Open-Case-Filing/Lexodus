use leptos::*;
use leptos_router::ActionForm;
use crate::layouts::wide::Wide_Layout;
use crate::models::case::Case;
use chrono::Utc;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use spin_sdk::{http_component, variables};

    }
}

#[server(AddCase, "/api")]
pub async fn add_case(
  case_number: String,
  title: String,
  status: String,
  filed_date: String,
  closed_date: Option<String>,
  court_name: String,
  current_court_name: String,
  judge_name: Option<String>,

) -> Result<String, ServerFnError> {
  let db_url = variables::get("db_url").unwrap();
println!("Hello params: {}", case_number);
 let conn = spin_sdk::pg::Connection::open(&db_url)?;

 let sql = "INSERT INTO cases (case_number, title, status, filed_date, closed_date, court_id, current_court_id, judge_id)
            VALUES ($1, $2, $3, $4, $5,
                    (SELECT id FROM courts WHERE name = $6),
                    (SELECT id FROM courts WHERE name = $7),
                    (SELECT id FROM judges WHERE name = $8))";

 let execute_result = conn.execute(
     sql,
     &[
         ParameterValue::Str(case_number.clone()),
         ParameterValue::Str(title.clone()),
         ParameterValue::Str(status.clone()),
         ParameterValue::Str(filed_date.clone()),
         ParameterValue::Str(closed_date.unwrap_or_default()),
         ParameterValue::Str(court_name.clone()),
         ParameterValue::Str(current_court_name.clone()),
         ParameterValue::Str(judge_name.unwrap_or_default()),
     ]);
 match execute_result {
     Ok(rows_affected) => {
         println!("Rows affected: {}", rows_affected);
         Ok(format!("Case added successfully: {}", rows_affected))
     },
     Err(e) => Err(ServerFnError::ServerError(format!("Failed to execute SQL: {}", e)))
 }


}

#[server(GetCases, "/api")]
pub async fn get_cases() -> Result<Vec<Case>, ServerFnError> {
    let case = vec![
        Case {
            id: 15,
            case_number: "2:22-cv-12345".to_string(),
            title: "Smith v. Tech Corp".to_string(),
            status: "Open".to_string(),
            filed_date: Utc::now(), // Use the actual filed_date from the database
            closed_date: None,
            court_id: 1,
            current_court_id: 1,
            judge_id: Some(1),
        },
    ];

    Ok(case)
}


#[island]
pub fn AddCaseForm() -> impl IntoView {
    let add_case = create_server_action::<AddCase>();
    let value = add_case.value();

    view! {
        <div class="add-case-form">
            <h2>"Add New Case"</h2>
            <ActionForm action=add_case>
                <div>
                    <label for="case_number">"Case Number:"</label>
                    <input type="text" id="case_number" name="case_number" required/>
                </div>
                <div>
                    <label for="title">"Title:"</label>
                    <input type="text" id="title" name="title" required/>
                </div>
                <div>
                    <label for="status">"Status:"</label>
                    <select id="status" name="status" required>
                        <option value="Open">"Open"</option>
                        <option value="Closed">"Closed"</option>
                        <option value="Pending">"Pending"</option>
                    </select>
                </div>
                <div>
                    <label for="filed_date">"Filed Date:"</label>
                    <input type="date" id="filed_date" name="filed_date" required/>
                </div>
                <div>
                    <label for="closed_date">"Closed Date:"</label>
                    <input type="date" id="closed_date" name="closed_date"/>
                </div>
                <div>
                    <label for="court_name">"Court Name:"</label>
                    <input type="text" id="court_name" name="court_name" required/>
                </div>
                <div>
                    <label for="current_court_name">"Current Court Name:"</label>
                    <input type="text" id="current_court_name" name="current_court_name" required/>
                </div>
                <div>
                    <label for="judge_name">"Judge Name:"</label>
                    <input type="text" id="judge_name" name="judge_name"/>
                </div>
                <button type="submit">"Add Case"</button>
            </ActionForm>
            <Show
                when=move || add_case.pending().get()
                fallback=|| view! { <div></div> }
            >
                <div>"Adding case..."</div>
            </Show>
            {move || value.get().map(|result| match result {
                Ok(case_number) => view! { <div>"Case added successfully. Case number: " {case_number}</div> },
                Err(_) => view! { <div>"Error adding case"</div> },
            })}
        </div>
    }
}





// Server-rendered component to display cases
#[component]
pub fn CaseList() -> impl IntoView {
    let cases = create_resource(|| (), |_| get_cases());

    view! {
        <div class="case-list">
            <h2>"Existing Cases"</h2>
            <Suspense fallback=move || view! { <p>"Loading cases..."</p> }>
                {move || cases.get().map(|result| match result {
                    Ok(cases) => view! {
                        <table>
                            <thead>
                                <tr>
                                    <th>"Case Number"</th>
                                    <th>"Title"</th>
                                    <th>"Status"</th>
                                    <th>"Filed Date"</th>
                                    <th>"Court"</th>
                                    <th>"Current Court"</th>
                                    <th>"Judge"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {cases.into_iter().map(|case| view! {
                                    <tr>
                                        <td>{case.case_number}</td>
                                        <td>{case.title}</td>
                                        <td>{case.status}</td>
                                        <td>{case.filed_date.to_string()}</td>
                                        <td>{case.court_id}</td>
                                        <td>{case.current_court_id}</td>
                                        <td>{case.judge_id.map_or_else(|| "-".to_string(), |id| id.to_string())}</td>
                                    </tr>
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    }.into_view(),
                    Err(_) => view! { <p>"Error loading cases"</p> }.into_view(),
                }).unwrap_or_else(|| view! { <p>"No cases loaded yet"</p> }.into_view())}
            </Suspense>
            <AddCaseForm />
        </div>
    }
}
