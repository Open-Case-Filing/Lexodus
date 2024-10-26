use crate::domain::models::user::SafeUser;
use crate::layouts::default::*;
use crate::providers::auth::AuthContext;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

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
        use crate::errors::LexodusAppError;
        use log::{info, error};

        fn case_number_exists(conn: &Connection, case_number: &str, filed_date: &str) -> Result<bool, ServerFnError> {
            let result = conn.query(
                "SELECT 1 FROM cases WHERE case_number = $1 AND filed_date = $2",
                &[
                    ParameterValue::Str(case_number.to_string()),
                    ParameterValue::Str(filed_date.to_string()),
                ]
            )?;

            Ok(!result.rows.is_empty())
        }

        fn generate_case_number(conn: &Connection, court_id: i64, filed_date: &str, judge_name: Option<&str>) -> Result<String, ServerFnError> {
            let year = filed_date[2..4].to_string(); // Extract year from filed_date (assuming format is YYYY-MM-DD)

            for attempt in 1..=100 {  // Increase max attempts
                // Get the latest case number for this court and year
                let result = conn.query(
                    "SELECT case_number
                     FROM cases
                     WHERE court_id = $1 AND case_number LIKE $2 || '%' AND filed_date = $3
                     ORDER BY REGEXP_REPLACE(case_number, '^(\\d+)-(\\d+)-(\\d+)-.*$', '\\3')::integer DESC
                     LIMIT 1",
                    &[
                        ParameterValue::Int64(court_id),
                        ParameterValue::Str(format!("{}-{}-", court_id, year)),
                        ParameterValue::Str(filed_date.to_string()),
                    ]
                )?;

                let new_sequence = if let Some(row) = result.rows.first() {
                    if let DbValue::Str(last_case_number) = &row[0] {
                        // Extract the sequence number and increment it
                        let parts: Vec<&str> = last_case_number.split('-').collect();
                        if parts.len() >= 3 {
                            if let Ok(seq) = parts[2].parse::<i64>() {
                                seq + 1
                            } else {
                                1
                            }
                        } else {
                            1
                        }
                    } else {
                        1
                    }
                } else {
                    1  // No existing cases for this court, year, and filed_date
                };

                let judge_identifier = match judge_name {
                    Some(name) => generate_judge_initials(name),
                    None => "XX".to_string()
                };

                let case_number = format!("{}-{}-{:04}-{}", court_id, year, new_sequence, judge_identifier);

                println!("Attempt {}: Trying to generate case number: {} for date: {}", attempt, case_number, filed_date);

                // Check if the generated case number already exists for this filed_date
                if !case_number_exists(conn, &case_number, filed_date)? {
                    return Ok(case_number);
                }

                println!("Case number {} already exists for date {}. Retrying...", case_number, filed_date);
            }

            Err(ServerFnError::ServerError(format!("Failed to generate unique case number after 100 attempts for court_id: {}, year: {}, filed_date: {}", court_id, year, filed_date)))
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


#[component]
pub fn CreateCaseForm(user: Option<SafeUser>) -> impl IntoView {
    let create_case = create_server_action::<CreateCase>();
    let response = create_case.value();

    let judges = create_resource(|| (), |_| get_judges());
    let courts = create_resource(|| (), |_| get_courts());

    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-6">"Create New Case"</h3>

            <ActionForm action=create_case class="space-y-6">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    // Title
                    <div>
                        <label for="title" class="block text-sm font-medium text-lexodus-700">"Title"</label>
                        <input type="text" id="title" name="title" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        />
                    </div>

                    // Case Type
                    <div>
                        <label for="case_type" class="block text-sm font-medium text-lexodus-700">"Case Type"</label>
                        <select id="case_type" name="case_type" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="CIVIL">"Civil"</option>
                            <option value="CRIMINAL">"Criminal"</option>
                            <option value="BANKRUPTCY">"Bankruptcy"</option>
                            <option value="ADMINISTRATIVE">"Administrative"</option>
                        </select>
                    </div>

                    // Nature of Suit
                    <div>
                        <label for="nature_of_suit" class="block text-sm font-medium text-lexodus-700">"Nature of Suit"</label>
                        <select id="nature_of_suit" name="nature_of_suit"
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="">"Select Nature of Suit"</option>
                            <option value="CONTRACT">"Contract"</option>
                            <option value="TORT">"Tort"</option>
                            <option value="CIVIL_RIGHTS">"Civil Rights"</option>
                            <option value="LABOR">"Labor"</option>
                            <option value="PROPERTY">"Property Rights"</option>
                            <option value="OTHER">"Other"</option>
                        </select>
                    </div>

                    // Filing Type
                    <div>
                        <label for="filing_type" class="block text-sm font-medium text-lexodus-700">"Filing Type"</label>
                        <select id="filing_type" name="filing_type" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="INITIAL">"Initial Filing"</option>
                            <option value="AMENDED">"Amended Filing"</option>
                            <option value="SUPPLEMENTAL">"Supplemental Filing"</option>
                        </select>
                    </div>

                    // Status
                    <div>
                        <label for="status" class="block text-sm font-medium text-lexodus-700">"Status"</label>
                        <select id="status" name="status" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="OPEN">"Open"</option>
                            <option value="PENDING">"Pending"</option>
                            <option value="CLOSED">"Closed"</option>
                            <option value="STAYED">"Stayed"</option>
                        </select>
                    </div>

                    // Filed Date
                    <div>
                        <label for="filed_date" class="block text-sm font-medium text-lexodus-700">"Filed Date"</label>
                        <input type="date" id="filed_date" name="filed_date" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        />
                    </div>

                    // Court
                    <div>
                        <label for="court_id" class="block text-sm font-medium text-lexodus-700">"Court"</label>
                        <select id="court_id" name="court_id" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
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

                    // Assigned Judge
                    <div>
                        <label for="assigned_judge_id" class="block text-sm font-medium text-lexodus-700">"Assigned Judge"</label>
                        <select id="assigned_judge_id" name="assigned_judge_id"
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="">"Select Judge (Optional)"</option>
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

                    // Security Level
                    <div>
                        <label for="security_level" class="block text-sm font-medium text-lexodus-700">"Security Level"</label>
                        <select id="security_level" name="security_level" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="PUBLIC">"Public"</option>
                            <option value="SEALED">"Sealed"</option>
                            <option value="RESTRICTED">"Restricted"</option>
                        </select>
                    </div>

                    // Jury Demand
                    <div>
                        <label for="jury_demand" class="block text-sm font-medium text-lexodus-700">"Jury Demand"</label>
                        <select id="jury_demand" name="jury_demand"
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="">"None"</option>
                            <option value="YES">"Yes"</option>
                            <option value="NO">"No"</option>
                        </select>
                    </div>

                    // Demand Amount
                    <div>
                        <label for="demand_amount" class="block text-sm font-medium text-lexodus-700">"Demand Amount"</label>
                        <input type="number" id="demand_amount" name="demand_amount" step="0.01" min="0"
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        />
                    </div>

                    // Jurisdictional Basis
                    <div>
                        <label for="jurisdictional_basis" class="block text-sm font-medium text-lexodus-700">"Jurisdictional Basis"</label>
                        <select id="jurisdictional_basis" name="jurisdictional_basis"
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="">"Select Basis"</option>
                            <option value="FEDERAL_QUESTION">"Federal Question"</option>
                            <option value="DIVERSITY">"Diversity"</option>
                            <option value="SUPPLEMENTAL">"Supplemental"</option>
                        </select>
                    </div>
                </div>

                <input type="hidden" name="user_id"
                    value=match user {
                        Some(u) => u.id.to_string(),
                        None => "-1".to_string(),
                    }
                />

                <div class="mt-6">
                    <button type="submit"
                        class="w-full inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-lexodus-600 hover:bg-lexodus-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-lexodus-500"
                    >
                        "Create Case"
                    </button>
                </div>
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
    let cases = create_resource(|| (), |_| async move { get_cases().await });

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
                                            <td colspan="8" class="text-center text-red-500 border-b py-4">"No existing cases found."</td>
                                        </tr>
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
                          <div class=move || format!("form-container px-4 sm:px-0 {}", if show_form.get() { "visible" } else { "" })>
                          <CreateCaseForm user=user()/>
                            </div>
                            <CaseList/>
                        </Show>
                      }
                  }}

                </Transition>


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
    pub user_id: String,
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
    court_id: String,
    case_type: String,
    nature_of_suit: Option<String>,
    filing_type: String,
    security_level: String,
    assigned_judge_id: Option<String>,
    jury_demand: Option<String>,
    jurisdictional_basis: Option<String>,
    user_id: String,
) -> Result<String, ServerFnError> {
    info!("Starting case creation process");

    // Parse user ID
    let user_id_i64 = user_id.parse::<i64>()
        .map_err(|_| LexodusAppError::BadRequest("Invalid user ID format".to_string()))?;

    // Parse court ID
    let court_id_i64 = court_id.parse::<i64>()
        .map_err(|_| LexodusAppError::BadRequest("Invalid court ID format".to_string()))?;

    // Parse judge ID if present
    let assigned_judge_id_i64 = if let Some(judge_id) = assigned_judge_id {
        if judge_id.is_empty() {
            None
        } else {
            Some(judge_id.parse::<i64>()
                .map_err(|_| LexodusAppError::BadRequest("Invalid judge ID format".to_string()))?)
        }
    } else {
        None
    };

    // Get database connection
    let db_url = variables::get("db_url")
        .map_err(|_| LexodusAppError::DBConnectionNotFound)?;

    let conn = Connection::open(&db_url)
        .map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    // Generate case number
    let case_number = generate_case_number(&conn, court_id_i64, &filed_date, None)?;

    let execute_result = if let Some(judge_id) = assigned_judge_id_i64 {
        // SQL with judge
        let sql = "INSERT INTO cases (
            case_number, title, case_type, nature_of_suit,
            filing_type, status, filed_date, court_id,
            assigned_judge_id, security_level, jury_demand,
            jurisdictional_basis, created_by, updated_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7::date, $8, $9, $10, $11, $12, $13, $13)
        RETURNING id";

        conn.execute(
            sql,
            &[
                ParameterValue::Str(case_number.clone()),
                ParameterValue::Str(title),
                ParameterValue::Str(case_type),
                ParameterValue::Str(nature_of_suit.unwrap_or_default()),
                ParameterValue::Str(filing_type),
                ParameterValue::Str(status),
                ParameterValue::Str(filed_date),
                ParameterValue::Int64(court_id_i64),
                ParameterValue::Int64(judge_id),
                ParameterValue::Str(security_level),
                ParameterValue::Str(jury_demand.unwrap_or_default()),
                ParameterValue::Str(jurisdictional_basis.unwrap_or_default()),
                ParameterValue::Int64(user_id_i64),
            ],
        )
    } else {
        // SQL without judge
        let sql = "INSERT INTO cases (
            case_number, title, case_type, nature_of_suit,
            filing_type, status, filed_date, court_id,
            security_level, jury_demand,
            jurisdictional_basis, created_by, updated_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7::date, $8, $9, $10, $11, $12, $12)
        RETURNING id";

        conn.execute(
            sql,
            &[
                ParameterValue::Str(case_number.clone()),
                ParameterValue::Str(title),
                ParameterValue::Str(case_type),
                ParameterValue::Str(nature_of_suit.unwrap_or_default()),
                ParameterValue::Str(filing_type),
                ParameterValue::Str(status),
                ParameterValue::Str(filed_date),
                ParameterValue::Int64(court_id_i64),
                ParameterValue::Str(security_level),
                ParameterValue::Str(jury_demand.unwrap_or_default()),
                ParameterValue::Str(jurisdictional_basis.unwrap_or_default()),
                ParameterValue::Int64(user_id_i64),
            ],
        )
    };

    match execute_result {
        Ok(_) => Ok(format!("Case created successfully with number {}", case_number)),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to create case: {}",
            e
        )))
    }
}

#[server(LogFailedCaseCreation, "/api")]
pub async fn log_failed_case_creation(
    reason: String,
    user_id: String,
) -> Result<(), ServerFnError> {
    let user_id_i64 = match user_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                "Invalid user ID format".to_string(),
            ))
        }
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
        ],
    )?;

    Ok(())
}
#[server(GetCases, "/api")]
pub async fn get_cases() -> Result<Vec<Case>, ServerFnError> {
  // Get database connection
  let db_url = variables::get("db_url")
      .map_err(|_| LexodusAppError::DBConnectionNotFound)?;
// Open Connection
  let conn = Connection::open(&db_url)
      .map_err(|e| LexodusAppError::DBError(e.to_string()))?;


    let sql = "SELECT c.id, c.case_number, c.title, c.status, c.filed_date,
               c.court_id, co.name as court_name,
               c.court_id as current_court_id, co.name as current_court_name,
               c.assigned_judge_id, j.name as judge_name,
               COALESCE(c.created_by::text, '-1') as user_id
               FROM cases c
               LEFT JOIN courts co ON c.court_id = co.id
               LEFT JOIN judicial_officers j ON c.assigned_judge_id = j.id
               ORDER BY c.filed_date DESC";

    let rowset = conn.query(sql, &[])
        .map_err(|e| LexodusAppError::DBError("Failed to execute query".to_string()))?;
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
                _ => "-1".to_string(), // Default value if user_id is not found
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
    // Get database connection
    let db_url = variables::get("db_url")
        .map_err(|_| LexodusAppError::DBConnectionNotFound)?;

    // Open Connection
    let conn = Connection::open(&db_url)
        .map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    let sql = "SELECT
                jo.id,
                u.full_name as name,
                jo.court_id
               FROM judicial_officers jo
               JOIN users u ON jo.user_id = u.id
               WHERE jo.status = 'ACTIVE'
               ORDER BY u.full_name";

    let rowset = conn.query(sql, &[])
        .map_err(|e| LexodusAppError::DBError("Failed to execute query".to_string()))?;

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
    // Get database connection
    let db_url = variables::get("db_url")
        .map_err(|_| LexodusAppError::DBConnectionNotFound)?;

    // Open Connection
    let conn = Connection::open(&db_url)
        .map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    let sql = "SELECT
                c.id,
                c.name,
                c.district,
                c.circuit
               FROM courts c
               ORDER BY c.name";

    let rowset = conn.query(sql, &[])
        .map_err(|e| LexodusAppError::DBError("Failed to execute query".to_string()))?;

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
