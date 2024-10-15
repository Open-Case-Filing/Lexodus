use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div>
      <h1>Home</h1>
      <a href="/case">Cases</a>
       <a href="/user">User</a>
      </div>

    }
}
