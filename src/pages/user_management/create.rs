use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use cfg_if::cfg_if;

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

#[server(AddUser, "/api")]
pub async fn add_user(
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
        },
        Err(e) => Err(ServerFnError::ServerError(format!("Failed to execute SQL: {}", e)))
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
  leptos_axum::redirect("/");
    Ok(users)
}


#[island]
pub fn AddUserForm() -> impl IntoView {
    let add_user = create_server_action::<AddUser>();
    let value = add_user.value();

    view! {
        <ErrorBoundary
            fallback=|errors| view! {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                    <strong class="font-bold">"Error adding user: "</strong>
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
            <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
                <h3 class="text-lg font-semibold mb-4 text-gray-300">"Add New User"</h3>
                <ActionForm action=add_user>

                <div class="mb-4">
                    <label for="username" class="block text-gray-400 mb-1">"Username:"</label>
                    <input type="text" id="username" name="username" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-gray-400 mb-1">"Password:"</label>
                    <input type="password" id="password" name="password" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <div class="mb-4">
                    <label for="role_id" class="block text-gray-400 mb-1">"Role ID:"</label>
                    <input type="number" id="role_id" name="role_id" class="w-full px-4 py-2 bg-gray-700 text-gray-300 rounded" required/>
                </div>
                <button type="submit" class="w-full px-4 py-2 bg-cyan-500 text-gray-900 rounded font-semibold hover:bg-cyan-600">"Add User"</button>
                </ActionForm>
                 <Show
                     when=move || add_user.pending().get()
                     fallback=|| view! { <div></div> }
                 >
                     <div class="mt-4 text-gray-400">"Adding user..."</div>
                 </Show>
                 {move || value.get().map(|result| match result {
                     Ok(message) => view! { <div class="mt-4 text-green-400">{message}</div> },
                     Err(e) => view! { <div class="mt-4 text-red-400">{e.to_string()}</div> },
                 })}
             </div>
         </ErrorBoundary>
     }
 }



#[component]
pub fn UserList() -> impl IntoView {
    let users = create_resource(|| (), |_| get_users());

    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Existing Users"</h3>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 text-gray-300 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"ID"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Username"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Role ID"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || users.get().map(|result| match result {
                            Ok(users) => users.into_iter().map(|user| view! {
                                <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                    <td class="border-t border-gray-700 px-4 py-2">{user.id}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{user.username}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{user.role_id}</td>
                                </tr>
                            }).collect_view(),
                            Err(_) => view! { <tr><td colspan="3" class="text-center text-red-400">"Error loading users"</td></tr> }.into_view(),
                        })}
                    </tbody>
                </table>
            </div>
            <AddUserForm />
        </div>
    }
}
