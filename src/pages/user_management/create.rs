use crate::layouts::default::*;
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
pub fn CreateUserForm() -> impl IntoView {
    let create_user = create_server_action::<CreateUser>();
    let value = create_user.value();

    let user_add_action = view! {
        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-6 rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-lexouds-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Add New User"</h3>
            <ActionForm action=create_user>
                <div class="mb-4">
                    <label for="username" class="block text-gray-400 mb-1">"Username:"</label>
                    <input type="text" id="username" name="username" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-gray-400 mb-1">"Password:"</label>
                    <input type="password" id="password" name="password" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <div class="mb-4">
                    <label for="role_id" class="block text-gray-400 mb-1">"Role ID:"</label>
                    <input type="number" id="role_id" name="role_id" class="w-full px-4 py-2 bg-gray-800 text-white rounded focus:outline-none" required/>
                </div>
                <button type="submit" class="w-full px-4 py-2 bg-lexouds-500 text-gray-900 rounded font-semibold hover:bg-lexouds-600">"Add User"</button>
            </ActionForm>
            <Show
                when=move || create_user.pending().get()
                fallback=|| view! { <div></div> }
            >
                <div class="mt-4 text-gray-400">"Adding user..."</div>
            </Show>
            {move || value.get().map(|result| match result {
                Ok(message) => view! { <div class="mt-4 text-green-400">{message}</div> },
                Err(e) => view! { <div class="mt-4 text-red-400">{e.to_string()}</div> },
            })}
        </div>
    };
    user_add_action
}

#[component]
pub fn UserList() -> impl IntoView {
    let (refresh_trigger, _set_refresh_trigger) = create_signal(0);
    let users = create_resource(
        move || refresh_trigger.get(),
        |_| async move { get_users().await },
    );
    let (user_trigger, _set_user_trigger) = create_signal(users.get());

    view! {
      <Transition
          fallback=move || view! { <p>"Loading users..."</p> }
      >
          <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-lexouds-500 mt-4">
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
                          <Suspense fallback=move || view! { <tr><td colspan="3" class="text-center">"Loading..."</td></tr> }>
                          {move || user_trigger.get().map(|result| match result {
                              Ok(users) => view! {
                                  <For
                                      each=move || users.clone()
                                      key=|user| user.id.clone()
                                      children=move |user| {
                                          view! {
                                              <tr class="hover:bg-lexouds-100 hover:text-gray-900">
                                                  <td class="border-t border-gray-700 px-4 py-2">{user.id}</td>
                                                  <td class="border-t border-gray-700 px-4 py-2">{user.username}</td>
                                                  <td class="border-t border-gray-700 px-4 py-2">{user.role_id}</td>
                                              </tr>
                                          }
                                      }
                                  />
                              }.into_view(),
                              Err(_) => view! {
                                  <tr><td colspan="3" class="text-center text-red-400">"Error loading users"</td></tr>
                              }.into_view(),
                          })}
                          </Suspense>
                      </tbody>
                  </table>
              </div>
          </div>
      </Transition>
    }
}

#[component]
pub fn UserManagement() -> impl IntoView {
    view! {
        <Meta property="og:title" content="User Management | Lexodus"/>
        <Title text="User Management | Lexodus"/>
        <Meta name="description" content="User management interface for OCFS with options to add, edit, and delete users."/>
        <Meta property="og:description" content="Manage users, roles, and permissions in the Lexodus."/>
        <DefaultLayout>
            <div class="w-full p-8">
                <div class="flex justify-between items-center mb-8">
                    <h2 class="text-2xl font-semibold">"User Management"</h2>
                </div>
                <UserList />
                <CreateUserForm />
            </div>
        </DefaultLayout>
    }
}
