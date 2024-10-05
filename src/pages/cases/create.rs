
use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use crate::layouts::default::*;
use leptos_meta::Meta;
use leptos_meta::Title;
// use crate::models::case::Case;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use spin_sdk::{variables};
        use spin_sdk::pg::*;

    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    case_number: String,
    title: String,
    status: String,
    filed_date: String,
    closed_date: Option<String>,
    court_id: i64,
    current_court_id: i64,
    judge_id: Option<i64>,
}

#[server(CreateCase, "/api")]
pub async fn create_case(
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
  let db_url = variables::get("db_url").unwrap();
  let conn = Connection::open(&db_url)?;

  let sql = "SELECT case_number, title, status, filed_date, closed_date, court_id, current_court_id, judge_id FROM cases";

  let rowset = conn.query(sql, &[])?;
  let cases: Vec<Case> = rowset
      .rows
      .iter()
      .map(|row| {
          Case {
              case_number: match &row[0] {
                  DbValue::Str(case_number) => case_number.clone(),
                  _ => String::new(), // Default value if not a String
              },
              title: match &row[1] {
                  DbValue::Str(title) => title.clone(),
                  _ => String::new(), // Default value if not a String
              },
              status: match &row[2] {
                  DbValue::Str(status) => status.clone(),
                  _ => String::new(), // Default value if not a String
              },
              filed_date: match &row[3] {
                  DbValue::Str(filed_date) => filed_date.clone(),
                  _ => String::new(), // Default value if not a String
              },
              closed_date: match &row[4] {
                  DbValue::Str(closed_date) => Some(closed_date.clone()),
                  DbValue::DbNull => None,
                  _ => None, // Default to None if not a String or Null
              },
              court_id: match &row[5] {
                  DbValue::Int64(court_id) => *court_id,
                  _ => 0, // Default value if not an Int64
              },
              current_court_id: match &row[6] {
                  DbValue::Int64(current_court_id) => *current_court_id,
                  _ => 0, // Default value if not an Int64
              },
              judge_id: match &row[7] {
                  DbValue::Int64(judge_id) => Some(*judge_id),
                  DbValue::DbNull => None,
                  _ => None, // Default to None if not an Int64 or Null
              },
          }
      })
      .collect();

  Ok(cases)

}

#[island]
pub fn CreateCaseForm() -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();
    let value = create_case.value();

    view! {
        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-6 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Add New Case"</h3>
            <ActionForm action=create_case>
                <div class="mb-4">
                    <label for="case_number" class="block text-gray-400 mb-1">"Case Number:"</label>
                    <input type="text" id="case_number" name="case_number" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="title" class="block text-gray-400 mb-1">"Title:"</label>
                    <input type="text" id="title" name="title" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="status" class="block text-gray-400 mb-1">"Status:"</label>
                    <select id="status" name="status" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required>
                        <option value="Open">"Open"</option>
                        <option value="Closed">"Closed"</option>
                        <option value="Pending">"Pending"</option>
                    </select>
                </div>
                <div class="mb-4">
                    <label for="filed_date" class="block text-gray-400 mb-1">"Filed Date:"</label>
                    <input type="date" id="filed_date" name="filed_date" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="closed_date" class="block text-gray-400 mb-1">"Closed Date:"</label>
                    <input type="date" id="closed_date" name="closed_date" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none"/>
                </div>
                <div class="mb-4">
                    <label for="court_id" class="block text-gray-400 mb-1">"Court ID:"</label>
                    <input type="number" id="court_id" name="court_id" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="current_court_id" class="block text-gray-400 mb-1">"Current Court ID:"</label>
                    <input type="number" id="current_court_id" name="current_court_id" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="judge_id" class="block text-gray-400 mb-1">"Judge ID:"</label>
                    <input type="number" id="judge_id" name="judge_id" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none"/>
                </div>
                <button type="submit" class="w-full px-4 py-2 bg-cyan-500 text-gray-900 rounded font-semibold hover:bg-cyan-600">"Add Case"</button>
            </ActionForm>
            <Show
                when=move || create_case.pending().get()
                fallback=|| view! { <div></div> }
            >
                <div class="mt-4 text-gray-400">"Adding case..."</div>
            </Show>
            {move || value.get().map(|result| match result {
                Ok(case_number) => view! { <div class="mt-4 text-green-400">"Case added successfully. Case number: " {case_number}</div> },
                Err(e) => view! { <div class="mt-4 text-red-400">{e.to_string()}</div> },
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
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 text-gray-300 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Number"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Title"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Filed Date"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Court ID"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Current Court ID"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Judge ID"</th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || cases.get().map(|result| match result {
                        Ok(cases) => cases.into_iter().map(|case| {
                            let case = case.clone();
                            view! {
                                <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                    <td class="border-t border-gray-700 px-4 py-2">{case.case_number}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.title}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.status}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.filed_date}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.court_id}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.current_court_id}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.judge_id.map_or_else(|| "-".to_string(), |id| id.to_string())}</td>
                                </tr>
                            }
                        }).collect_view(),
                        Err(e) => view! { <tr><td colspan="7" class="text-center text-red-400">{e.to_string()}</td></tr> }.into_view(),
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn CaseManagement() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Case Management | Lexodus"/>
        <Title text="Case Management | Lexodus"/>
        <Meta name="description" content="Case management interface for OCFS with options to add, view, and manage cases."/>
        <Meta property="og:description" content="Add new cases and view existing cases in the Lexodus."/>
        <Default_Layout>
            <div class="w-full p-8">
                <div class="flex justify-between items-center mb-8">
                    <h2 class="text-2xl font-semibold">"Case Management"</h2>
                </div>
                <CaseList />
                <CreateCaseForm />
            </div>
        </Default_Layout>
    }
}
