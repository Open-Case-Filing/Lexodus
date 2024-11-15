use crate::domain::models::{case::CreateCaseParams, user::SafeUser};
use crate::layouts::default::*;
use crate::providers::auth::AuthContext;
use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue, DbValue};
        use spin_sdk::{variables};
        use crate::errors::LexodusAppError;
        use chrono::NaiveDate;
        use log::info;

        fn case_number_exists(conn: &Connection, case_number: &str, filed_date: &str) -> Result<bool, ServerFnError> {
            let sql = r#"
                SELECT 1 FROM cases
                WHERE case_number = $1
                AND filed_date = TO_DATE($2, 'YYYY-MM-DD')"#;  // Convert the date here

            let result = conn.query(
                sql,
                &[
                    ParameterValue::Str(case_number.to_string()),
                    ParameterValue::Str(filed_date.to_string()),
                ]
            )?;

            Ok(!result.rows.is_empty())
        }

        fn generate_case_number(conn: &Connection, court_id: i64, filed_date: &str, judge_name: Option<&str>) -> Result<String, ServerFnError> {
            let year = filed_date[2..4].to_string(); // Extract year from filed_date (assuming format is YYYY-MM-DD)
            let sql = r#"
                SELECT case_number
                FROM cases
                WHERE court_id = $1
                AND case_number LIKE $2 || '%'
                AND filed_date = TO_DATE($3, 'YYYY-MM-DD')  -- Convert the date here
                ORDER BY REGEXP_REPLACE(case_number, '^(\d+)-(\d+)-(\d+)-.*$', '\3')::integer DESC
                LIMIT 1"#;
            for attempt in 1..=100 {  // Increase max attempts
                // Get the latest case number for this court and year
                let result = conn.query(
                  sql,
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
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8">
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
                        </select>
                    </div>

                    // Filing Type
                    <div>
                        <label for="filing_type" class="block text-sm font-medium text-lexodus-700">"Filing Type"</label>
                        <select id="filing_type" name="filing_type" required
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        >
                            <option value="ELECTRONIC">"Electronic"</option>
                            <option value="PAPER">"Paper"</option>
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
                                        view! { <option value={court.id.to_string()}>{court.name}</option> }
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
                                        view! { <option value={judge.id.to_string()}>{judge.name}</option> }
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
                        </select>
                    </div>

                    // Demand Amount
                    <div>
                        <label for="demand_amount" class="block text-sm font-medium text-lexodus-700">"Demand Amount"</label>
                        <input type="number" id="demand_amount" name="demand_amount" step="0.01" min="0"
                            class="mt-1 block w-full rounded-md border border-lexodus-300 shadow-sm focus:border-lexodus-500 focus:ring-lexodus-500"
                        />
                    </div>
                </div>

                <input type="hidden" name="user_id"
                    value=match user {
                        Some(u) => u.id.to_string(),
                        None => "-1".to_string(),
                    }
                />
                <input
                    type="hidden"
                    name="status"
                    value="PENDING"  // Or whatever default status you want
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
        <div class="mt-8">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-4">"Cases"</h3>
            <div class="bg-white shadow-lg border border-lexodus-200 rounded-lg overflow-hidden">
                <table class="min-w-full">
                    <thead>
                        <tr class="bg-lexodus-50">
                            <th class="px-6 py-3 text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">"Case Number"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">"Title"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">"Status"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">"Filed Date"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-lexodus-700 uppercase tracking-wider">"Court"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-lexodus-200">
                        <Suspense fallback=move || view! { <tr><td colspan="5" class="px-6 py-4 text-center">"Loading..."</td></tr> }>
                            {move || cases.get().map(|result| match result {
                                Ok(cases) => cases.into_iter().map(|case| {
                                    view! {
                                        <tr class="hover:bg-lexodus-50">
                                            <td class="px-6 py-4 text-sm">{case.case_number}</td>
                                            <td class="px-6 py-4 text-sm">{case.title}</td>
                                            <td class="px-6 py-4 text-sm">{case.status}</td>
                                            <td class="px-6 py-4 text-sm">{case.filed_date}</td>
                                            <td class="px-6 py-4 text-sm">{case.court_name}</td>
                                        </tr>
                                    }
                                }).collect_view(),
                                Err(e) => view! {
                                    <tr>
                                        <td colspan="5" class="px-6 py-4 text-center text-red-500">{e.to_string()}</td>
                                    </tr>
                                }.into_view(),
                            })}
                        </Suspense>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn CaseManagement() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    view! {
        <DefaultLayout>
            <div class="p-8">
                <h2 class="text-2xl font-semibold text-lexodus-800 mb-8">"Case Management"</h2>
                <Transition fallback=move || ()>
                    {move || {
                        match auth_context.user.get() {
                            Some(Ok(Some(user))) => view! {
                                <CreateCaseForm user=Some(user.clone())/>
                                <CaseList/>
                            }.into_view(),
                            _ => view! { <div>"Please log in to access case management."</div> }.into_view(),
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
    demand_amount: Option<String>,
) -> Result<String, ServerFnError> {
    info!("Starting case creation process");

    // Validate status
    let status = status.to_uppercase();
    if !["PENDING", "ACTIVE", "CLOSED"].contains(&status.as_str()) {
        return Err(ServerFnError::ServerError(
            "Invalid case status".to_string(),
        ));
    }

    // Parse and validate the date
    let parsed_date = NaiveDate::parse_from_str(&filed_date, "%Y-%m-%d")
        .map_err(|_| LexodusAppError::BadRequest("Invalid date format".to_string()))?;
    let formatted_date = parsed_date.format("%Y-%m-%d").to_string();

    // Parse other parameters
    let user_id_i64 = user_id
        .parse::<i64>()
        .map_err(|_| LexodusAppError::BadRequest("Invalid user ID format".to_string()))?;

    let court_id_i64 = court_id
        .parse::<i64>()
        .map_err(|_| LexodusAppError::BadRequest("Invalid court ID format".to_string()))?;

    let assigned_judge_id_i64 =
        if let Some(judge_id) = assigned_judge_id {
            if judge_id.is_empty() {
                None
            } else {
                Some(judge_id.parse::<i64>().map_err(|_| {
                    LexodusAppError::BadRequest("Invalid judge ID format".to_string())
                })?)
            }
        } else {
            None
        };

    let demand_amount_f64 = if let Some(amount) = demand_amount {
        if amount.is_empty() {
            None
        } else {
            Some(amount.parse::<f64>().map_err(|_| {
                LexodusAppError::BadRequest("Invalid demand amount format".to_string())
            })?)
        }
    } else {
        None
    };

    let db_url = variables::get("db_url").map_err(|_| LexodusAppError::DBConnectionNotFound)?;
    let conn = Connection::open(&db_url).map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    conn.execute("BEGIN", &[])?;

    let case_number = generate_case_number(&conn, court_id_i64, &formatted_date, None)?;

    let (sql, parameters) = if let Some(judge_id) = assigned_judge_id_i64 {
        (
            r#"
            INSERT INTO cases (
                case_number, title, case_type, nature_of_suit,
                filing_type, status, filed_date, court_id,
                security_level, jury_demand, jurisdictional_basis,
                created_by, demand_amount, sealed, assigned_judge_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                TO_DATE($7, 'YYYY-MM-DD'),
                $8, $9, $10, $11, $12,
                TO_NUMBER($13, '999999999999999D99'),
                false, $14
            )
            RETURNING id"#,
            vec![
                ParameterValue::Str(case_number.clone()),
                ParameterValue::Str(title),
                ParameterValue::Str(case_type),
                ParameterValue::Str(nature_of_suit.unwrap_or_default()),
                ParameterValue::Str(filing_type),
                ParameterValue::Str(status.clone()),
                ParameterValue::Str(filed_date.clone()),
                ParameterValue::Int64(court_id_i64),
                ParameterValue::Str(security_level),
                ParameterValue::Str(jury_demand.unwrap_or_default()),
                ParameterValue::Str(jurisdictional_basis.unwrap_or_default()),
                ParameterValue::Int64(user_id_i64),
                ParameterValue::Str(demand_amount_f64.unwrap_or(0.0).to_string()),
                ParameterValue::Int64(judge_id),
            ],
        )
    } else {
        (
            r#"
            INSERT INTO cases (
                case_number, title, case_type, nature_of_suit,
                filing_type, status, filed_date, court_id,
                security_level, jury_demand, jurisdictional_basis,
                created_by, demand_amount, sealed
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                TO_DATE($7, 'YYYY-MM-DD'),
                $8, $9, $10, $11, $12,
                TO_NUMBER($13, '999999999999999D99'),
                false
            )
            RETURNING id"#,
            vec![
                ParameterValue::Str(case_number.clone()),
                ParameterValue::Str(title),
                ParameterValue::Str(case_type),
                ParameterValue::Str(nature_of_suit.unwrap_or_default()),
                ParameterValue::Str(filing_type),
                ParameterValue::Str(status.clone()),
                ParameterValue::Str(filed_date.clone()),
                ParameterValue::Int64(court_id_i64),
                ParameterValue::Str(security_level),
                ParameterValue::Str(jury_demand.unwrap_or_default()),
                ParameterValue::Str(jurisdictional_basis.unwrap_or_default()),
                ParameterValue::Int64(user_id_i64),
                ParameterValue::Str(demand_amount_f64.unwrap_or(0.0).to_string()),
            ],
        )
    };

    let result = conn.query(sql, &parameters);

    match result {
        Ok(_) => {
            // Insert into case_status_history
            let status_sql = r#"
                INSERT INTO case_status_history (
                    case_id, case_filed_date, old_status, new_status,
                    changed_by, change_date, notes
                )
                SELECT
                    id,
                    TO_DATE($1, 'YYYY-MM-DD'),
                    'DRAFT',
                    $2,
                    $3,
                    NOW(),
                    'Initial case filing'
                FROM cases
                WHERE case_number = $4"#;

            conn.execute(
                status_sql,
                &[
                    ParameterValue::Str(formatted_date),
                    ParameterValue::Str(status),
                    ParameterValue::Int64(user_id_i64),
                    ParameterValue::Str(case_number.clone()),
                ],
            )?;

            if let Some(amount) = demand_amount_f64 {
                let demand_sql = r#"
                    INSERT INTO case_events (
                        case_id, case_filed_date, event_type_id,
                        event_date, title, description,
                        filed_by, entered_by, event_status,
                        metadata
                    )
                    SELECT
                        c.id,
                        c.filed_date,
                        et.id,
                        NOW(),
                        'Demand Amount Set',
                        'Initial demand amount set to ' || $1::text,
                        $2,
                        $2,
                        'ACTIVE',
                        jsonb_build_object('amount', $1::float8)
                    FROM cases c
                    CROSS JOIN (
                        SELECT id FROM event_types
                        WHERE name = 'FILING'
                        LIMIT 1
                    ) et
                    WHERE c.case_number = $3"#;

                conn.execute(
                    demand_sql,
                    &[
                        ParameterValue::Str(amount.to_string()),
                        ParameterValue::Int64(user_id_i64),
                        ParameterValue::Str(case_number.clone()),
                    ],
                )?;
            }

            conn.execute("COMMIT", &[])?;
            Ok(format!(
                "Case created successfully with number {}",
                case_number
            ))
        }
        Err(e) => {
            conn.execute("ROLLBACK", &[])?;
            Err(ServerFnError::ServerError(format!(
                "Failed to create case: {}",
                e
            )))
        }
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
    let db_url = variables::get("db_url").map_err(|_| LexodusAppError::DBConnectionNotFound)?;
    let conn = Connection::open(&db_url).map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    let sql = "SELECT DISTINCT
               c.id, c.case_number, c.title, c.status, c.filed_date,
               c.court_id, co.name as court_name,
               c.court_id as current_court_id, co.name as current_court_name,
               c.assigned_judge_id,
               u.full_name as judge_name,  -- Changed from j.name to u.full_name
               COALESCE(c.created_by::text, '-1') as user_id
               FROM cases c
               LEFT JOIN courts co ON c.court_id = co.id
               LEFT JOIN judicial_officers jo ON c.assigned_judge_id = jo.id
               LEFT JOIN users u ON jo.user_id = u.id  -- Added join with users table
               WHERE c.filed_date IS NOT NULL
               ORDER BY c.filed_date DESC";

    let cases: Vec<Case> = conn
        .query(sql, &[])
        .map_err(|e| LexodusAppError::DBError(format!("Failed to execute query: {}", e)))?
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
                _ => "-1".to_string(),
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
    let db_url = variables::get("db_url").map_err(|_| LexodusAppError::DBConnectionNotFound)?;

    // Open Connection
    let conn = Connection::open(&db_url).map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    let sql = "SELECT
                jo.id,
                u.full_name as name,
                jo.court_id
               FROM judicial_officers jo
               JOIN users u ON jo.user_id = u.id
               WHERE jo.status = 'ACTIVE'
               ORDER BY u.full_name";

    let rowset = conn
        .query(sql, &[])
        .map_err(|_e| LexodusAppError::DBError("Failed to execute query".to_string()))?;
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
    let db_url = variables::get("db_url").map_err(|_| LexodusAppError::DBConnectionNotFound)?;

    // Open Connection
    let conn = Connection::open(&db_url).map_err(|e| LexodusAppError::DBError(e.to_string()))?;

    let sql = "SELECT
                c.id,
                c.name,
                c.district,
                c.circuit
               FROM courts c
               ORDER BY c.name";

    let rowset = conn
        .query(sql, &[])
        .map_err(|_e| LexodusAppError::DBError("Failed to execute query".to_string()))?;

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
