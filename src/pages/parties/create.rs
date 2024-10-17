use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use spin_sdk::{variables};
        use spin_sdk::pg::*;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Party {
    id: i64,
    name: String,
    role: String,
    attorney_name: String,
    case_filed_date: String,
}

#[component]
pub fn PartiesManagement(case_id: i64) -> impl IntoView {
    let parties = create_resource(move || case_id, |id| get_parties(id));
    let add_party = create_server_action::<AddParty>();

    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8">
            <h2 class="text-xl font-semibold text-lexodus-800 mb-6">"Case Parties"</h2>

            <table class="min-w-full bg-white">
                <thead>
                    <tr>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Name"</th>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Role"</th>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Attorney"</th>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Case Filed Date"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || parties.get().map(|result| match result {
                        Ok(parties) => parties.into_iter().map(|party| {
                            view! {
                                <tr class="hover:bg-lexodus-50">
                                    <td class="py-2 px-4 border-b text-lexodus-800">{party.name}</td>
                                    <td class="py-2 px-4 border-b text-lexodus-800">{party.role}</td>
                                    <td class="py-2 px-4 border-b text-lexodus-800">{party.attorney_name}</td>
                                    <td class="py-2 px-4 border-b text-lexodus-800">{party.case_filed_date}</td>
                                </tr>
                            }
                        }).collect_view(),
                        Err(e) => view! {
                            <tr>
                                <td colspan="4" class="text-center text-red-500 border-b py-4">{e.to_string()}</td>
                            </tr>
                        }.into_view(),
                    })}
                </tbody>
            </table>

            <div class="mt-8">
                <h3 class="text-lg font-semibold text-lexodus-800 mb-4">"Add New Party"</h3>
                <ActionForm action=add_party>
                    <input type="hidden" name="case_id" value=case_id.to_string()/>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <input
                            type="text"
                            name="name"
                            placeholder="Party Name"
                            class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexodus-500"
                            required
                        />
                        <input
                            type="text"
                            name="role"
                            placeholder="Role"
                            class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexodus-500"
                            required
                        />
                        <input
                            type="number"
                            name="attorney_id"
                            placeholder="Attorney ID (optional)"
                            class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexodus-500"
                        />
                        <input
                            type="date"
                            name="case_filed_date"
                            placeholder="Case Filed Date"
                            class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexodus-500"
                            required
                        />
                    </div>
                    <button
                        type="submit"
                        class="mt-4 px-4 py-2 bg-lexodus-500 text-white rounded font-semibold hover:bg-lexodus-600 focus:outline-none focus:ring-2 focus:ring-lexodus-500"
                    >
                        "Add Party"
                    </button>
                </ActionForm>
            </div>
        </section>
    }
}

#[server(GetParties, "/api")]
pub async fn get_parties(case_id: i64) -> Result<Vec<Party>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT p.id, p.name, p.role, u.username AS attorney_name, p.case_filed_date
               FROM parties p
               LEFT JOIN users u ON p.attorney_id = u.id
               WHERE p.case_id = $1";

    let rowset = conn.query(sql, &[ParameterValue::Int64(case_id)])?;
    let parties: Vec<Party> = rowset
        .rows
        .iter()
        .map(|row| Party {
            id: match &row[0] {
                DbValue::Int64(id) => *id,
                _ => 0,
            },
            name: match &row[1] {
                DbValue::Str(name) => name.clone(),
                _ => String::new(),
            },
            role: match &row[2] {
                DbValue::Str(role) => role.clone(),
                _ => String::new(),
            },
            attorney_name: match &row[3] {
                DbValue::Str(attorney_name) => attorney_name.clone(),
                _ => String::new(),
            },
            case_filed_date: match &row[4] {
                DbValue::Str(date) => date.clone(),
                _ => String::new(),
            },
        })
        .collect();

    Ok(parties)
}

#[server(AddParty, "/api")]
pub async fn add_party(
    case_id: i64,
    name: String,
    role: String,
    attorney_id: Option<i64>,
    case_filed_date: String,
) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO parties (case_id, name, role, attorney_id, case_filed_date) VALUES ($1, $2, $3, $4, $5)";

    let execute_result = conn.execute(
        sql,
        &[
            ParameterValue::Int64(case_id),
            ParameterValue::Str(name),
            ParameterValue::Str(role),
            attorney_id.map_or(ParameterValue::DbNull, ParameterValue::Int64),
            ParameterValue::Str(case_filed_date),
        ],
    );

    match execute_result {
        Ok(rows_affected) => {
            println!("Rows affected: {}", rows_affected);
            Ok(format!("Party added successfully: {}", rows_affected))
        }
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to execute SQL: {}",
            e
        ))),
    }
}
