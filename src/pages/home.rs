use leptos::*;
use crate::layouts::default::DefaultLayout;
#[component]
pub fn Home() -> impl IntoView {
    view! {
      <DefaultLayout>
      <h1>Home</h1>
      <a href="/cases">Cases</a>
       <a href="/users">User</a>
      </DefaultLayout>

    }
}
