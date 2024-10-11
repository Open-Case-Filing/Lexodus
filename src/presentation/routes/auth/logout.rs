use crate::functions;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Logout(action: Action<functions::auth::Logout, Result<(), ServerFnError>>) -> impl IntoView {
    _ = &action.dispatch(functions::auth::Logout {});

    view! {
      <Meta property="og:title" content="Logout"/>
      <Title text="Logout"/>
      <Meta name="description" content="Logout"/>
      <Meta property="og:description" content="Logout"/>

      />
      <Redirect path="/"/>
    }
}
