
use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use crate::layouts::default::*;
use leptos_meta::Meta;
use leptos_meta::Title;




use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use spin_sdk::{variables};
        use spin_sdk::pg::*;


    }
}


#[component]
pub fn CreateCaseForm() -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();
    let value = create_case.value();

    view! {
      // Section Container with Lexodus Style
      <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
          <h3 class="text-xl font-semibold text-lexodus-800 mb-6">"Add New Case"</h3>

          // Form for Adding New Case
          <ActionForm action=create_case>

              // Case Number Input
              <div class="mb-4">
                  <label for="case_number" class="block text-lexodus-700 mb-1">"Case Number:"</label>
                  <input type="text" id="case_number" name="case_number" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
              </div>

              // Title Input
              <div class="mb-4">
                  <label for="title" class="block text-lexodus-700 mb-1">"Title:"</label>
                  <input type="text" id="title" name="title" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
              </div>

              // Status Dropdown
              <div class="mb-4">
                  <label for="status" class="block text-lexodus-700 mb-1">"Status:"</label>
                  <select id="status" name="status" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required>
                      <option value="Open">"Open"</option>
                      <option value="Closed">"Closed"</option>
                      <option value="Pending">"Pending"</option>
                  </select>
              </div>

              // Filed Date Input
              <div class="mb-4">
                  <label for="filed_date" class="block text-lexodus-700 mb-1">"Filed Date:"</label>
                  <input type="date" id="filed_date" name="filed_date" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
              </div>

              // Closed Date Input
              <div class="mb-4">
                  <label for="closed_date" class="block text-lexodus-700 mb-1">"Closed Date:"</label>
                  <input type="date" id="closed_date" name="closed_date" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
              </div>

              // Court ID Input
              <div class="mb-4">
                  <label for="court_id" class="block text-lexodus-700 mb-1">"Court ID:"</label>
                  <input type="number" id="court_id" name="court_id" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
              </div>

              // Current Court ID Input
              <div class="mb-4">
                  <label for="current_court_id" class="block text-lexodus-700 mb-1">"Current Court ID:"</label>
                  <input type="number" id="current_court_id" name="current_court_id" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
              </div>

              // Judge ID Input
              <div class="mb-4">
                  <label for="judge_id" class="block text-lexodus-700 mb-1">"Judge ID:"</label>
                  <input type="number" id="judge_id" name="judge_id" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
              </div>

              // Submit Button
              <button type="submit" class="w-full px-4 py-2 bg-lexodus-500 text-white rounded font-semibold hover:bg-lexodus-600 focus:outline-none focus:ring-2 focus:ring-lexodus-500">"Add Case"</button>
          </ActionForm>

          // Loading State for Form Submission
          <Show
              when=move || create_case.pending().get()
              fallback=|| view! { <div></div> }
          >
              <div class="mt-4 text-lexodus-700">"Adding case..."</div>
          </Show>

          // Feedback Message for Success or Error
          {move || value.get().map(|result| match result {
              Ok(case_number) => view! {
                  <div class="mt-4 text-green-500">"Case added successfully. Case number: " {case_number}</div>
              },
              Err(e) => view! {
                  <div class="mt-4 text-red-500">{e.to_string()}</div>
              },
          })}
      </section>
    }
}

#[component]
pub fn CaseList() -> impl IntoView {
    let cases = create_resource(|| (), |_| get_cases());

    view! {
              // Outer Container for Existing Cases
              <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
                  <h2 class="text-xl font-semibold text-lexodus-800 mb-6">"Existing Cases"</h2>

                  // Table for Case Data
                  <table class="min-w-full bg-white">
                      <thead>
                          <tr>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Case Number"</th>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Title"</th>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Status"</th>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Filed Date"</th>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Court ID"</th>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Current Court ID"</th>
                              <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Judge ID"</th>
                          </tr>
                      </thead>
                      <tbody>
                          // Dynamically Render Cases or Show Error
                          {move || cases.get().map(|result| match result {
                              Ok(cases) => cases.into_iter().map(|case| {
                                  let case = case.clone();
                                  view! {
                                      // Table Row with Hover Effect
                                      <tr class="hover:bg-lexodus-50">
                                          <td class="py-2 px-4 border-b text-lexodus-800">{case.case_number}</td>
                                          <td class="py-2 px-4 border-b text-lexodus-800">{case.title}</td>
                                          <td class="py-2 px-4 border-b text-lexodus-800">
                                              <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                                                  {case.status}
                                              </span>
                                          </td>
                                          <td class="py-2 px-4 border-b text-lexodus-800">{case.filed_date}</td>
                                          <td class="py-2 px-4 border-b text-lexodus-800">{case.court_id}</td>
                                          <td class="py-2 px-4 border-b text-lexodus-800">{case.current_court_id}</td>
                                          <td class="py-2 px-4 border-b text-lexodus-800">
                                              {case.judge_id.map_or_else(|| "-".to_string(), |id| id.to_string())}
                                          </td>
                                      </tr>
                                  }
                              }).collect_view(),

                              // Error Row for Failed Case Retrieval
                              Err(e) => view! {
                                  <tr>
                                      <td colspan="7" class="text-center text-red-500 border-b py-4">{e.to_string()}</td>
                                  </tr>
                              }.into_view(),
                          })}
                      </tbody>
                  </table>
              </section>
          }
      }



#[component]
pub fn CaseManagement() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Case Management | Lexodus"/>
        <Title text="Case Management | Lexodus"/>
        <Meta name="description" content="Case management interface for OCFS with options to add, view, and manage cases."/>
        <Meta property="og:description" content="Add new cases and view existing cases in the Lexodus."/>
        <DefaultLayout>
            <div class="w-full p-8">
                <div class="flex justify-between items-center mb-8">
                    <h2 class="text-2xl font-semibold">"Case Management"</h2>
                </div>
                <CaseList />
                <CreateCaseForm />
            </div>
        </DefaultLayout>
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
