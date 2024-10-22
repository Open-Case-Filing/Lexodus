use crate::layouts::default::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use crate::domain::models::user::SafeUser;
use crate::providers::auth::AuthContext;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue, DbValue};
        use spin_sdk::{variables};
        use std::collections::HashMap;
        use std::sync::Arc;
        use spin_sdk::http::{Request, Headers};
        use chrono::Utc;
        use rand::Rng;



        fn generate_case_number(conn: &Connection, court_id: i64, judge_name: Option<&str>) -> Result<String, ServerFnError> {
            let now = Utc::now();
            let year = now.format("%y").to_string();

            // Get the highest sequence number for this court and year
            let result = conn.query(
                "SELECT COALESCE(
                    MAX(REGEXP_REPLACE(case_number, '^(\\d+)-(\\d+)-(\\d+)-.*$', '\\3')::integer
                ), 0)
                FROM cases
                WHERE case_number LIKE $1 || '-' || $2 || '-%'",
                &[
                    &ParameterValue::Int64(court_id),
                    &ParameterValue::Str(year.clone())  // Clone here
                ]
            )?;

            let max_sequence = if let Some(row) = result.rows.first() {
                match &row[0] {
                    DbValue::Int64(seq) => *seq,
                    _ => 0,
                }
            } else {
                0
            };

            let new_sequence = max_sequence + 1;

            let judge_identifier = match judge_name {
                Some(name) => generate_judge_initials(name),
                None => "XX".to_string()
            };

            Ok(format!("{}-{}-{:04}-{}", court_id, year, new_sequence, judge_identifier))
        }


        fn generate_judge_initials(name: &str) -> String {
            let mut initials = name
                .split_whitespace()
                .filter_map(|word| word.chars().next())
                .take(2)
                .map(|c| c.to_ascii_uppercase())
                .collect::<String>();

            // If we don't have 2 initials, pad with 'X'
            while initials.len() < 2 {
                initials.push('X');
            }

            initials
        }
    }
}



#[cfg(feature = "ssr")]
fn get_client_ip(req: &Request) -> String {
    let forwarded = req.header("x-forwarded-for")
        .and_then(|v| v.as_str())  // as_str() already returns Option<&str>
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim())
        .unwrap_or_else(||
            req.header("x-real-ip")
                .and_then(|v| v.as_str())  // as_str() already returns Option<&str>
                .map(|s| s.trim())
                .unwrap_or("unknown")
        )
        .to_string();

    forwarded
}

#[cfg(feature = "ssr")]
fn get_user_agent(req: &Request) -> String {
    req.header("user-agent")
        .and_then(|v| v.as_str())  // as_str() already returns Option<&str>
        .unwrap_or("unknown")
        .to_string()
}


#[component]
pub fn CreateCaseForm(user: Option<SafeUser>) -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();
    let response = create_case.value();

    let judges = create_resource(|| (), |_| get_judges());
    let courts = create_resource(|| (), |_| get_courts());



    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-6">"Create New Case"</h3>

            <ActionForm action=create_case>
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
                <input
                  type="hidden"
                  name="user_id"
                  value=match user {
                      Some(u) => u.id,
                      None => -1,
                  }
                />
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
        <div class="mt-8 -mx-4 sm:mx-0">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-4 px-4 sm:px-0">"Existing Cases"</h3>
            <div class="bg-white shadow-lg border border-lexodus-200 overflow-hidden sm:rounded-lg">
                <div class="overflow-x-auto">
                    <table class="w-full bg-white table-auto">
                        <thead>
                            <tr>
                                <th class="py-2 pl-4 pr-2 sm:px-4 border-b text-left text-lexodus-700 font-medium">"Case Number"</th>
                                <th class="py-2 px-2 sm:px-4 border-b text-left text-lexodus-700 font-medium">"Title"</th>
                                <th class="py-2 px-2 sm:px-4 border-b text-left text-lexodus-700 font-medium">"Status"</th>
                                <th class="hidden sm:table-cell py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Filed Date"</th>
                                <th class="hidden sm:table-cell py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Court"</th>
                                <th class="hidden sm:table-cell py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Current Court"</th>
                                <th class="hidden sm:table-cell py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Judge"</th>
                                <th class="hidden sm:table-cell py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Actions"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <Suspense fallback=move || view! { <tr><td colspan="8" class="text-center py-4">"Loading..."</td></tr> }>
                                {move || cases.get().map(|result| match result {
                                    Ok(cases) => cases.into_iter().map(|case| {
                                        view! {
                                            <>
                                                <tr class="hover:bg-lexodus-50">
                                                    <td class="py-2 pl-4 pr-2 sm:px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">{case.case_number}</td>
                                                    <td class="py-2 px-2 sm:px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">{case.title}</td>
                                                    <td class="py-2 px-2 sm:px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">{case.status}</td>
                                                    <td class="hidden sm:table-cell py-2 px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">{case.filed_date}</td>
                                                    <td class="hidden sm:table-cell py-2 px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">{case.court_name}</td>
                                                    <td class="hidden sm:table-cell py-2 px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">{case.current_court_name}</td>
                                                    <td class="hidden sm:table-cell py-2 px-4 border-b text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">
                                                        {case.judge_name.unwrap_or_else(|| "Not assigned".to_string())}
                                                    </td>
                                                    <td class="hidden sm:table-cell py-2 px-4 border-b text-sm text-lexodus-800">
                                                        <button
                                                            class="bg-lexodus-500 text-white px-3 py-1 rounded hover:bg-lexodus-600"
                                                            on:click=move |_| {
                                                                // TODO: Implement view case details
                                                            }
                                                        >
                                                            "View"
                                                        </button>
                                                    </td>
                                                </tr>
                                                <tr class="sm:hidden hover:bg-lexodus-50">
                                                    <td colspan="3" class="py-2 px-4 border-b text-right">
                                                        <button
                                                            class="w-full bg-lexodus-500 text-white px-4 py-2 rounded hover:bg-lexodus-600 text-sm font-medium"
                                                            on:click=move |_| {
                                                                // TODO: Implement view case details
                                                            }
                                                        >
                                                            "View Case Details"
                                                        </button>
                                                    </td>
                                                </tr>
                                            </>
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
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CaseManagement() -> impl IntoView {
  let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let (show_form, set_show_form) = create_signal(false);
    view! {
        <Meta property="og:title" content="Case Management | Lexodus"/>
        <Title text="Case Management | Lexodus"/>
        <Meta name="description" content="Efficient case management interface for Lexodus"/>
        <Meta property="og:description" content="Manage legal cases efficiently in the Lexodus system"/>
        <DefaultLayout>
            <div class="w-full px-0 sm:p-8 bg-lexodus-50">
                <div class="flex justify-between items-center mb-8 px-4 sm:px-0">
                    <h2 class="text-xl sm:text-2xl font-semibold text-lexodus-800">"Case Management"</h2>
                    <button
                        class="bg-lexodus-600 text-white text-sm sm:text-base px-2 py-1 sm:px-4 sm:py-2 rounded hover:bg-lexodus-700"
                        on:click=move |_| set_show_form.update(|v| *v = !*v)
                    >
                        "Create New Case"
                    </button>
                </div>
                <div class=move || format!("form-container px-4 sm:px-0 {}", if show_form.get() { "visible" } else { "" })>
                <Transition fallback=move || ()>
                  {move || {
                      let user = move || {
                          match auth_context.user.get() {
                              Some(Ok(Some(user))) => Some(user),
                              Some(Ok(None)) => None,
                              Some(Err(_)) => None,
                              None => None,
                          }
                      };
                      view! {
                        <Show when=move || user().is_some() fallback=|| ().into_view()>
                          <CreateCaseForm user=user()/>
                        </Show>
                      }
                  }}

                </Transition>

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
    pub user_id: String
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
    title: String,
    status: String,
    filed_date: String,
    court_id: i64,
    judge_id: Option<i64>,
    user_id: String,
) -> Result<String, ServerFnError> {
    // Convert user_id to i64
    let user_id_i64 = match user_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return Err(ServerFnError::ServerError("Invalid user ID format".to_string()))
    };

    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    // Set the context BEFORE doing any operations
    conn.execute(
        "SELECT
            set_config('app.current_user_id', $1, true),
            set_config('app.current_ip_address', $2, true),
            set_config('app.current_user_agent', $3, true)",
        &[
            ParameterValue::Str(user_id.clone()),
            ParameterValue::Str("0.0.0.0".to_string()),  // or get real IP if available
            ParameterValue::Str("Lexodus Web Client".to_string()),  // or get real user agent
        ]
    )?;

    // Verify the context was set
    conn.execute(
        "DO $$
        BEGIN
            RAISE NOTICE 'Context set: user_id=%, ip=%, agent=%',
                current_setting('app.current_user_id', true),
                current_setting('app.current_ip_address', true),
                current_setting('app.current_user_agent', true);
        END $$;",
        &[]
    )?;

    // First fetch user's role and permissions
    let user_info = conn.query(
        "SELECT r.name as role_name
         FROM users u
         JOIN roles r ON r.id = u.role_id
         WHERE u.id = $1",
        &[ParameterValue::Int64(user_id_i64)]
    )?;

    // Check if user exists and get their role
    let role = if let Some(row) = user_info.rows.first() {
        match &row[0] {
            DbValue::Str(role) => role.clone(),
            _ => return Err(ServerFnError::ServerError("Invalid role data".to_string()))
        }
    } else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    };

    // Check create_case permission
    let has_permission = conn.query(
        "SELECT 1
         FROM users u
         JOIN role_permissions rp ON rp.role_id = u.role_id
         JOIN permissions p ON p.id = rp.permission_id
         WHERE u.id = $1
         AND p.name = 'create_case'",
        &[ParameterValue::Int64(user_id_i64)]
    )?;

    if has_permission.rows.is_empty() {
        return Err(ServerFnError::ServerError(
            "Unauthorized: Insufficient permissions to create case".to_string()
        ));
    }

    // Input validation
    if title.is_empty() || status.is_empty() || filed_date.is_empty() {
        return Err(ServerFnError::ServerError("Invalid input: Required fields missing".to_string()));
    }

    // Generate unique case number within transaction
    let case_number = generate_case_number(&conn, court_id, None)?;

    let sql = if judge_id.is_some() {
        "INSERT INTO cases (
            case_number, title, status, filed_date,
            court_id, current_court_id, judge_id,
            created_by, created_by_role
         )
         VALUES ($1, $2, $3, $4, $5, $5, $6, $7, $8)
         RETURNING id"
    } else {
        "INSERT INTO cases (
            case_number, title, status, filed_date,
            court_id, current_court_id,
            created_by, created_by_role
         )
         VALUES ($1, $2, $3, $4, $5, $5, $6, $7)
         RETURNING id"
    };// Inside create_case function:
    let sql = if judge_id.is_some() {
        "INSERT INTO cases (
            case_number, title, status, filed_date,
            court_id, current_court_id, judge_id,
            created_by, created_by_role
         )
         VALUES ($1, $2, $3, $4, $5, $5, $6, $7, $8)
         RETURNING id"
    } else {
        "INSERT INTO cases (
            case_number, title, status, filed_date,
            court_id, current_court_id,
            created_by, created_by_role
         )
         VALUES ($1, $2, $3, $4, $5, $5, $6, $7)
         RETURNING id"
    };

    let params = if let Some(judge) = judge_id {
        vec![
            ParameterValue::Str(case_number.clone()),
            ParameterValue::Str(title),
            ParameterValue::Str(status),
            ParameterValue::Str(filed_date),
            ParameterValue::Int64(court_id),
            ParameterValue::Int64(judge),
            ParameterValue::Int64(user_id_i64),
            ParameterValue::Str(role),
        ]
    } else {
        vec![
            ParameterValue::Str(case_number.clone()),
            ParameterValue::Str(title),
            ParameterValue::Str(status),
            ParameterValue::Str(filed_date),
            ParameterValue::Int64(court_id),
            ParameterValue::Int64(user_id_i64),
            ParameterValue::Str(role),
        ]
    };

    let result = conn.query(sql, &params)?;

    let result = conn.query(sql, &params)?;

    // Final verify of context after insert
    let final_check = conn.query(
        "SELECT
            current_setting('app.current_user_id', true) as user_id,
            current_setting('app.current_ip_address', true) as ip,
            current_setting('app.current_user_agent', true) as agent",
        &[]
    )?;

    let case_id = if let Some(row) = result.rows.first() {
        match row[0] {
            DbValue::Int64(id) => id,
            _ => return Err(ServerFnError::ServerError("Failed to get case ID".to_string())),
        }
    } else {
        return Err(ServerFnError::ServerError("No case ID returned".to_string()));
    };

    Ok(format!("Case created successfully with number {}", case_number))
}

#[server(LogFailedCaseCreation, "/api")]
pub async fn log_failed_case_creation(
    reason: String,
    user_id: String,
) -> Result<(), ServerFnError> {
    let user_id_i64 = match user_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return Err(ServerFnError::ServerError("Invalid user ID format".to_string()))
    };

    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    conn.execute(
        "INSERT INTO failed_operations (user_id, operation, reason, timestamp)
         VALUES ($1, $2, $3, NOW())",
        &[
            ParameterValue::Int64(user_id_i64),
            ParameterValue::Str("create_case".to_string()),
            ParameterValue::Str(reason),
        ]
    )?;

    Ok(())
}
#[server(GetCases, "/api")]
pub async fn get_cases() -> Result<Vec<Case>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT c.id, c.case_number, c.title, c.status, c.filed_date,
               c.court_id, co.name as court_name,
               c.current_court_id, cco.name as current_court_name,
               c.judge_id, j.name as judge_name,
               COALESCE(c.user_id, '-1') as user_id
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
            user_id: match &row[11] {
                DbValue::Str(user_id) => user_id.clone(),
                _ => "-1".to_string(),  // Default value if user_id is not found
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
