use leptos::*;
use crate::domain::models::User;
use crate::api::get_users;

#[component]
pub fn UserList() -> impl IntoView {
    let users = create_resource(|| async move {
        get_users().await.unwrap_or_default()
    });

    view! {
        <div>
            <h2>"User List"</h2>
            <ul>
                {move || users.read().map(|users| {
                    users.iter().map(|user| view! {
                        <li>{format!("{} - {}", user.id, user.username)}</li>
                    }).collect_view()
                })}
            </ul>
        </div>
    }
}
