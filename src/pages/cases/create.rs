
use leptos::*;
use leptos_router::ActionForm;

use crate::models::case::Case;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use spin_sdk::{variables};
        use chrono::Utc;

    }
}

#[server(AddCase, "/api")]
pub async fn add_case(
    case_number: String,
    title: String,
    status: String,
    filed_date: String,
    closed_date: Option<String>,
    court_id: i64,
    current_court_id: i64,
    judge_id: Option<i64>,
) -> Result<String, ServerFnError> {
    println!("--> Adding a new case: {}", filed_date);
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO cases (case_number, title, status, filed_date, closed_date, court_id, current_court_id, judge_id)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

    let execute_result = conn.execute(
        sql,
        &[
            ParameterValue::Str(case_number),
            ParameterValue::Str(title),
            ParameterValue::Str(status),
            ParameterValue::Str(filed_date),
            closed_date.map(ParameterValue::Str).unwrap_or(ParameterValue::DbNull),
            ParameterValue::Int64(court_id),
            ParameterValue::Int64(current_court_id),
            judge_id.map(ParameterValue::Int64).unwrap_or(ParameterValue::DbNull),
        ],
    );

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

        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Add New Case"</h3>
            <ActionForm action=add_case>
                <div class="mb-4">
                    <label for="case_number" class="block text-gray-400 mb-1">"Case Number:"</label>
                    <input type="text" id="case_number" name="case_number" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="title" class="block text-gray-400 mb-1">"Title:"</label>
                    <input type="text" id="title" name="title" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="status" class="block text-gray-400 mb-1">"Status:"</label>
                    <select id="status" name="status" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required>
                        <option value="Open">"Open"</option>
                        <option value="Closed">"Closed"</option>
                        <option value="Pending">"Pending"</option>
                    </select>
                </div>
                <div class="mb-4">
                    <label for="filed_date" class="block text-gray-400 mb-1">"Filed Date:"</label>
                    <input type="date" id="filed_date" name="filed_date" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="closed_date" class="block text-gray-400 mb-1">"Closed Date:"</label>
                    <input type="date" id="closed_date" name="closed_date" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded"/>
                </div>
                <div class="mb-4">
                    <label for="court_id" class="block text-gray-400 mb-1">"Court Name:"</label>
                    <input type="text" id="court_id" name="court_id" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="current_court_id" class="block text-gray-400 mb-1">"Current Court Name:"</label>
                    <input type="text" id="current_court_id" name="current_court_id" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="judge_id" class="block text-gray-400 mb-1">"Judge Name:"</label>
                    <input type="text" id="judge_id" name="judge_id" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded"/>
                </div>
                <button type="submit" class="w-full px-4 py-2 bg-cyan-500 text-gray-900 rounded font-semibold hover:bg-cyan-600">"Add Case"</button>
            </ActionForm>
            <Show
                when=move || add_case.pending().get()
                fallback=|| view! { <div></div> }
            >
                <div class="mt-4 text-gray-400">"Adding case..."</div>
            </Show>
            {move || value.get().map(|result| match result {
                Ok(case_number) => view! { <div class="mt-4 text-green-400">"Case added successfully. Case number: " {case_number}</div> },
                Err(_) => view! { <div class="mt-4 text-red-400">"Error adding case"</div> },
            })}
        </div>

    }
}






#[component]
pub fn CaseList() -> impl IntoView {
    let cases = create_resource(|| (), |_| get_cases());

    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Existing Cases"</h3>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                        <strong class="font-bold">"Error loading cases: "</strong>
                        <ul class="list-disc list-inside">
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <div class="overflow-x-auto">
                    <table class="min-w-full bg-gray-800 text-gray-300 hover:table-fixed">
                        <thead>
                            <tr>
                                <th class="px-4 py-2 text-left text-gray-400">"Case Number"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Title"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Filed Date"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Court"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Current Court"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Judge"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || cases.get().map(|result| match result {
                                Ok(cases) => cases.into_iter().map(|case| view! {
                                    <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                        <td class="border-t border-gray-700 px-4 py-2">{case.case_number}</td>
                                        <td class="border-t border-gray-700 px-4 py-2">{case.title}</td>
                                        <td class="border-t border-gray-700 px-4 py-2">{case.status}</td>
                                        <td class="border-t border-gray-700 px-4 py-2">{case.filed_date.to_string()}</td>
                                        <td class="border-t border-gray-700 px-4 py-2">{case.court_id}</td>
                                        <td class="border-t border-gray-700 px-4 py-2">{case.current_court_id}</td>
                                        <td class="border-t border-gray-700 px-4 py-2">{case.judge_id.map_or_else(|| "-".to_string(), |id| id.to_string())}</td>
                                    </tr>
                                }).collect_view(),
                                Err(e) => view! { <tr><td colspan="7" class="text-center text-red-400">{e.to_string()}</td></tr> }.into_view(),
                            })}
                        </tbody>
                    </table>
                </div>
            </ErrorBoundary>
            <AddCaseForm />
        </div>
    }
}