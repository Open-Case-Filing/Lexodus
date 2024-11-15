use crate::domain::models::user::SafeUser;
use crate::layouts::default::*;
use crate::providers::auth::AuthContext;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::Meta;
use leptos_meta::Title;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::*;
        use spin_sdk::variables;

    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role_id: i64,
}

#[server(CreateUser, "/api")]
pub async fn create_user(
    username: String,
    password: String,
    role_id: i64,
) -> Result<String, ServerFnError> {
    println!("--> Adding a new user: {}", username);

    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO users (username, password_hash, role_id)
               VALUES ($1, $2, $3)";

    let execute_result = conn.execute(
        sql,
        &[
            ParameterValue::Str(username),
            ParameterValue::Str(password), // Note: In a real app, you should hash the password
            ParameterValue::Int64(role_id),
        ],
    );

    match execute_result {
        Ok(rows_affected) => {
            println!("Rows affected: {}", rows_affected);
            Ok(format!("User added successfully: {}", rows_affected))
        }
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to execute SQL: {}",
            e
        ))),
    }
}

#[server(GetUsers, "/api")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, username, role_id FROM users";

    let rowset = conn.query(sql, &[])?;
    let users: Vec<User> = rowset
        .rows
        .iter()
        .map(|row| {
            User {
                id: match &row[0] {
                    DbValue::Int64(id) => *id,
                    _ => 0, // Default value if not an Int64
                },
                username: match &row[1] {
                    DbValue::Str(username) => username.clone(),
                    _ => String::new(), // Default value if not a String
                },
                role_id: match &row[2] {
                    DbValue::Int64(role_id) => *role_id,
                    _ => 0, // Default value if not an Int64
                },
            }
        })
        .collect();

    Ok(users)
}
#[component]
pub fn CreateUserForm(user: Option<SafeUser>) -> impl IntoView {
    let create_user = create_server_action::<CreateUser>();
    let response = create_user.value();

    let has_error = move || response.with(|val| matches!(val, Some(Err(_))));

    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
            <h3 class="text-xl font-semibold text-lexodus-800 mb-6">"Add New User"</h3>

            <ActionForm action=create_user>
            <input type="hidden" name="user_id"
                value=match user {
                    Some(u) => u.id.to_string(),
                    None => "-1".to_string(),
                }
            />q
                <div class="mb-4">
                    <label for="username" class="block text-lexodus-700 mb-1">"Username:"</label>
                    <input type="text" id="username" name="username" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-lexodus-700 mb-1">"Password:"</label>
                    <input type="password" id="password" name="password" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
                </div>
                <div class="mb-4">
                    <label for="role_id" class="block text-lexodus-700 mb-1">"Role ID:"</label>
                    <input type="number" id="role_id" name="role_id" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500" required/>
                </div>
                <button type="submit" class="w-full px-4 py-2 bg-lexodus-500 text-white rounded font-semibold hover:bg-lexodus-600 focus:outline-none focus:ring-2 focus:ring-lexodus-500">"Add User"</button>
            </ActionForm>

            <Show
                when=move || create_user.pending().get()
                fallback=|| view! { <div></div> }
            >
                <div class="mt-4 text-lexodus-700">"Adding user..."</div>
            </Show>

            {move || response.get().map(|result| match result {
                Ok(message) => view! { <div class="mt-4 text-green-500">{message}</div> },
                Err(e) => view! { <div class="mt-4 text-red-500">{e.to_string()}</div> },
            })}

            {move || has_error().then(|| view! {
                <p class="mt-4 text-red-500">"An error occurred while creating the user."</p>
            })}
        </section>
    }
}
#[component]
pub fn UserList() -> impl IntoView {
    let users = create_resource(|| (), |_| get_users());

    view! {
        <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
            <h2 class="text-xl font-semibold text-lexodus-800 mb-6">"Existing Users"</h2>

            <table class="min-w-full bg-white">
                <thead>
                    <tr>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"ID"</th>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Username"</th>
                        <th class="py-2 px-4 border-b text-left text-lexodus-700 font-medium">"Role ID"</th>
                    </tr>
                </thead>
                <tbody>
                    <Suspense fallback=move || view! { <tr><td colspan="3" class="text-center py-4">"Loading..."</td></tr> }>
                    {move || users.get().map(|result| match result {
                        Ok(users) => users.into_iter().map(|user| {
                            view! {
                                <tr class="hover:bg-lexodus-50">
                                    <td class="py-2 px-4 border-b text-lexodus-800">{user.id}</td>
                                    <td class="py-2 px-4 border-b text-lexodus-800">{user.username}</td>
                                    <td class="py-2 px-4 border-b text-lexodus-800">{user.role_id}</td>
                                </tr>
                            }
                        }).collect_view(),
                        Err(e) => view! {
                            <tr>
                                <td colspan="3" class="text-center text-red-500 border-b py-4">{e.to_string()}</td>
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
pub fn UserManagement() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    view! {
        <Meta property="og:title" content="User Management | Lexodus"/>
        <Title text="User Management | Lexodus"/>
        <Meta name="description" content="User management interface for OCFS with options to add, edit, and delete users."/>
        <Meta property="og:description" content="Manage users, roles, and permissions in the Lexodus."/>
        <DefaultLayout>
            <div class="w-full p-8 bg-lexodus-50">
                <div class="flex justify-between items-center mb-8">
                    <h2 class="text-2xl font-semibold text-lexodus-800">"User Management"</h2>
                </div>
                <div class="space-y-6">
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
                          <UserList/>
                          <CreateUserForm user=user()/>
                        </Show>
                      }
                  }}

                </Transition>

                </div>
            </div>
        </DefaultLayout>
    }
}
