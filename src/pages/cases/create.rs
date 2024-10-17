use crate::layouts::default::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::{use_navigate, ActionForm};
use serde::{Deserialize, Serialize};

use crate::pages::parties::create::PartiesManagement;

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
    let response = create_case.value();

    let judges = create_resource(|| (), |_| get_judges());
    let courts = create_resource(|| (), |_| get_courts());

    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-6">"Create New Case"</h3>

            <ActionForm action=create_case>
                <div class="mb-4">
                    <label for="case_number" class="block text-lexodus-700 mb-1">"Case Number:"</label>
                    <input type="text" id="case_number" name="case_number" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
                </div>
                <div class="mb-4">
                    <label for="title" class="block text-lexodus-700 mb-1">"Title:"</label>
                    <input type="text" id="title" name="title" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
                </div>
                <div class="mb-4">
                    <label for="status" class="block text-lexodus-700 mb-1">"Status:"</label>
                    <select id="status" name="status" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required>
                        <option value="Open">"Open"</option>
                        <option value="Closed">"Closed"</option>
                        <option value="Pending">"Pending"</option>
                    </select>
                </div>
                <div class="mb-4">
                    <label for="filed_date" class="block text-lexodus-700 mb-1">"Filed Date:"</label>
                    <input type="date" id="filed_date" name="filed_date" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
                </div>
                <div class="mb-4">
                    <label for="court_id" class="block text-lexodus-700 mb-1">"Court:"</label>
                    <select id="court_id" name="court_id" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required>
                        <Suspense fallback=move || view! { <option>"Loading courts..."</option> }>
                            {move || courts.get().map(|result| match result {
                                Ok(courts) => courts.into_iter().map(|court| {
                                    view! {
                                        <option value={court.id.to_string()}>{court.name} " - " {court.district}</option>
                                    }
                                }).collect_view(),
                                Err(_) => view! { <option>"Failed to load courts"</option> }.into_view(),
                            })}
                        </Suspense>
                    </select>
                </div>
                <div class="mb-4">
                    <label for="judge_id" class="block text-lexodus-700 mb-1">"Judge (Optional):"</label>
                    <select id="judge_id" name="judge_id" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500">
                        <option value="">"No judge assigned"</option>
                        <Suspense fallback=move || view! { <option>"Loading judges..."</option> }>
                            {move || judges.get().map(|result| match result {
                                Ok(judges) => judges.into_iter().map(|judge| {
                                    view! {
                                        <option value={judge.id.to_string()}>{judge.name}</option>
                                    }
                                }).collect_view(),
                                Err(_) => view! { <option>"Failed to load judges"</option> }.into_view(),
                            })}
                        </Suspense>
                    </select>
                </div>
                <button type="submit" class="w-full px-4 py-2 bg-lexodus-500 text-white rounded font-semibold hover:bg-lexodus-600 focus:outline-none focus:ring-2 focus:ring-lexodus-500">"Create Case"</button>
            </ActionForm>

            <Show
                when=move || create_case.pending().get()
                fallback=|| view! { <div></div> }
            >
                <div class="mt-4 text-lexodus-700">"Creating case..."</div>
            </Show>

            {move || response.get().map(|result| match result {
                Ok(message) => view! { <div class="mt-4 text-green-500">{message}</div> },
                Err(e) => view! { <div class="mt-4 text-red-500">{e.to_string()}</div> },
            })}
        </section>
    }
}

#[component]
pub fn CaseList() -> impl IntoView {
    let cases = create_resource(|| (), |_| get_cases());

    view! {
    <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
        <h2 class="text-xl font-semibold text-lexodus-800 mb-6">"Existing Cases"</h2>

        <table class="min-w-full bg-white">
        <thead>
            <tr>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Case Number"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Title"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Status"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Filed Date"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Court"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Current Court"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Judge"</th>
                <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Actions"</th>
            </tr>
        </thead>
            <tbody>
                <Suspense fallback=move || view! { <tr><td colspan="5" class="text-center py-4">"Loading..."</td></tr> }>
                {move || cases.get().map(|result| match result {
                    Ok(cases) => cases.into_iter().map(|case| {
                        view! {
                            <tr class="hover:bg-lexodus-50">
                                <td class="py-2 px-4 border-b text-lexodus-800">{case.case_number}</td>
                                <td class="py-2 px-4 border-b text-lexodus-800">{case.title}</td>
                                <td class="py-2 px-4 border-b text-lexodus-800">{case.status}</td>
                                <td class="py-2 px-4 border-b text-lexodus-800">{case.filed_date}</td>
                                <td class="py-2 px-4 border-b text-lexodus-800">{case.court_name}</td>
                                <td class="py-2 px-4 border-b text-lexodus-800">{case.current_court_name}</td>
                                <td class="py-2 px-4 border-b text-lexodus-800">
                                    {case.judge_name.unwrap_or_else(|| "Not assigned".to_string())}
                                </td>
                                <td class="py-2 px-4 border-b text-lexodus-800">
                                    <button class="bg-lexodus-500 text-white px-3 py-1 rounded hover:bg-lexodus-600"
                                            on:click=move |_| {
                                                // TODO: Implement view case details
                                            }>
                                        "View"
                                    </button>
                                </td>
                            </tr>
                                                            }
                                             }).collect_view(),
                             Err(e) => view! {
                                   <tr>
                                         <td colspan="8" class="text-center text-red-500 border-b py-4">{e.to_string()}</td>
                                     </tr>
                               }.into_view(),
                         })}
                         </Suspense>
                     </tbody>
                 </table>
              </section>
                                    }
}




#[component]
pub fn CaseManagement() -> impl IntoView {
    let (show_form, set_show_form) = create_signal(false);

    view! {
        <Meta property="og:title" content="Case Management | Lexodus"/>
        <Title text="Case Management | Lexodus"/>
        <Meta name="description" content="Efficient case management interface for Lexodus"/>
        <Meta property="og:description" content="Manage legal cases efficiently in the Lexodus system"/>
        <DefaultLayout>
            <div class="w-full p-8 bg-lexodus-50">
                <div class="flex justify-between items-center mb-8">
                    <h2 class="text-2xl font-semibold text-lexodus-800">"Case Management"</h2>
                    <button
                        class="bg-lexodus-600 text-white px-4 py-2 rounded hover:bg-lexodus-700"
                        on:click=move |_| set_show_form.update(|v| *v = !*v)

                    >
                        "Create New Case"
                    </button>
                </div>

                <div class=move || format!("form-container {}", if show_form.get() { "visible" } else { "" })>
                    <CreateCaseForm />
                </div>

                <CaseList />
            </div>
        </DefaultLayout>
    }
}


#[component]
fn QuickActions() -> impl IntoView {
    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-4">"Quick Actions"</h3>
            <div class="space-y-2">
                <a href="/cases/file-motion" class="block w-full py-2 px-4 bg-lexodus-500 text-white text-center rounded hover:bg-lexodus-600">"File a Motion"</a>
                <a href="/cases/schedule-hearing" class="block w-full py-2 px-4 bg-lexodus-500 text-white text-center rounded hover:bg-lexodus-600">"Schedule a Hearing"</a>
                <a href="/cases/upload-document" class="block w-full py-2 px-4 bg-lexodus-500 text-white text-center rounded hover:bg-lexodus-600">"Upload Document"</a>
            </div>
        </section>
    }
}

#[component]
fn UpcomingEvents() -> impl IntoView {
    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-4">"Upcoming Events"</h3>
            <ul class="space-y-4">
                <EventItem date="2023-09-01" event="Status Conference" />
                <EventItem date="2023-09-15" event="Pretrial Hearing" />
                <EventItem date="2023-10-01" event="Trial Start Date" />
            </ul>
        </section>
    }
}

#[component]
fn EventItem(date: &'static str, event: &'static str) -> impl IntoView {
    view! {
        <li class="flex items-center">
            <div class="w-16 text-sm text-lexodus-600">{date}</div>
            <div class="flex-grow font-medium">{event}</div>
        </li>
    }
}

#[component]
fn RecentActivity() -> impl IntoView {
    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-4">"Recent Activity"</h3>
            <ul class="space-y-4">
                <ActivityItem action="Document Filed" details="Motion for Summary Judgment" />
                <ActivityItem action="Hearing Scheduled" details="Status Conference on 2023-09-01" />
                <ActivityItem action="Order Issued" details="Motion to Compel Discovery Granted" />
            </ul>
        </section>
    }
}

#[component]
fn ActivityItem(action: &'static str, details: &'static str) -> impl IntoView {
    view! {
        <li>
            <p class="font-medium">{action}</p>
            <p class="text-sm text-lexodus-600">{details}</p>
        </li>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Case {
    pub id: i64,
    pub case_number: String,
    pub title: String,
    pub status: String,
    pub filed_date: String,
    pub court_id: i64,
    pub court_name: String,
    pub current_court_id: i64,
    pub current_court_name: String,
    pub judge_id: Option<i64>,
    pub judge_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Judge {
    pub id: i64,
    pub name: String,
    pub court_id: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Court {
    pub id: i64,
    pub name: String,
    pub district: String,
    pub circuit: String,
}

#[server(CreateCase, "/api")]
pub async fn create_case(
    case_number: String,
    title: String,
    status: String,
    filed_date: String,
    court_id: i64,
    judge_id: Option<i64>,
) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = if judge_id.is_some() {
        "INSERT INTO cases (case_number, title, status, filed_date, court_id, current_court_id, judge_id)
         VALUES ($1, $2, $3, $4, $5, $5, $6)"
    } else {
        "INSERT INTO cases (case_number, title, status, filed_date, court_id, current_court_id)
         VALUES ($1, $2, $3, $4, $5, $5)"
    };

    let execute_result = if let Some(judge) = judge_id {
        conn.execute(
            sql,
            &[
                ParameterValue::Str(case_number),
                ParameterValue::Str(title),
                ParameterValue::Str(status),
                ParameterValue::Str(filed_date),
                ParameterValue::Int64(court_id),
                ParameterValue::Int64(judge),
            ],
        )
    } else {
        conn.execute(
            sql,
            &[
                ParameterValue::Str(case_number),
                ParameterValue::Str(title),
                ParameterValue::Str(status),
                ParameterValue::Str(filed_date),
                ParameterValue::Int64(court_id),
            ],
        )
    };

    match execute_result {
        Ok(rows_affected) => Ok(format!("Case created successfully: {}", rows_affected)),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to create case: {}",
            e
        ))),
    }
}

#[server(GetCases, "/api")]
pub async fn get_cases() -> Result<Vec<Case>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT c.id, c.case_number, c.title, c.status, c.filed_date,
               c.court_id, co.name as court_name,
               c.current_court_id, cco.name as current_court_name,
               c.judge_id, j.name as judge_name
               FROM cases c
               LEFT JOIN courts co ON c.court_id = co.id
               LEFT JOIN courts cco ON c.current_court_id = cco.id
               LEFT JOIN judges j ON c.judge_id = j.id";

    let rowset = conn.query(sql, &[])?;
    let cases: Vec<Case> = rowset
        .rows
        .iter()
        .map(|row| Case {
            id: match &row[0] {
                DbValue::Int64(id) => *id,
                _ => 0,
            },
            case_number: match &row[1] {
                DbValue::Str(case_number) => case_number.clone(),
                _ => String::new(),
            },
            title: match &row[2] {
                DbValue::Str(title) => title.clone(),
                _ => String::new(),
            },
            status: match &row[3] {
                DbValue::Str(status) => status.clone(),
                _ => String::new(),
            },
            filed_date: match &row[4] {
                DbValue::Str(filed_date) => filed_date.clone(),
                _ => String::new(),
            },
            court_id: match &row[5] {
                DbValue::Int64(court_id) => *court_id,
                _ => 0,
            },
            court_name: match &row[6] {
                DbValue::Str(court_name) => court_name.clone(),
                _ => String::new(),
            },
            current_court_id: match &row[7] {
                DbValue::Int64(current_court_id) => *current_court_id,
                _ => 0,
            },
            current_court_name: match &row[8] {
                DbValue::Str(current_court_name) => current_court_name.clone(),
                _ => String::new(),
            },
            judge_id: match &row[9] {
                DbValue::Int64(judge_id) => Some(*judge_id),
                _ => None,
            },
            judge_name: match &row[10] {
                DbValue::Str(judge_name) => Some(judge_name.clone()),
                _ => None,
            },
        })
        .collect();

    Ok(cases)
}

#[server(UpdateCaseStatus, "/api")]
pub async fn update_case_status(case_id: i64, new_status: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE cases SET status = $1 WHERE id = $2";

    let execute_result = conn.execute(
        sql,
        &[
            ParameterValue::Str(new_status),
            ParameterValue::Int64(case_id),
        ],
    );

    match execute_result {
        Ok(rows_affected) => Ok(format!(
            "Case status updated successfully: {}",
            rows_affected
        )),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to update case status: {}",
            e
        ))),
    }
}

#[server(GetJudges, "/api")]
pub async fn get_judges() -> Result<Vec<Judge>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, name, court_id FROM judges";

    let rowset = conn.query(sql, &[])?;
    let judges: Vec<Judge> = rowset
        .rows
        .iter()
        .map(|row| Judge {
            id: match &row[0] {
                DbValue::Int64(id) => *id,
                _ => 0,
            },
            name: match &row[1] {
                DbValue::Str(name) => name.clone(),
                _ => String::new(),
            },
            court_id: match &row[2] {
                DbValue::Int64(court_id) => *court_id,
                _ => 0,
            },
        })
        .collect();

    Ok(judges)
}

#[server(GetCourts, "/api")]
pub async fn get_courts() -> Result<Vec<Court>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, name, district, circuit FROM courts";

    let rowset = conn.query(sql, &[])?;
    let courts: Vec<Court> = rowset
        .rows
        .iter()
        .map(|row| Court {
            id: match &row[0] {
                DbValue::Int64(id) => *id,
                _ => 0,
            },
            name: match &row[1] {
                DbValue::Str(name) => name.clone(),
                _ => String::new(),
            },
            district: match &row[2] {
                DbValue::Str(district) => district.clone(),
                _ => String::new(),
            },
            circuit: match &row[3] {
                DbValue::Str(circuit) => circuit.clone(),
                _ => String::new(),
            },
        })
        .collect();

    Ok(courts)
}
