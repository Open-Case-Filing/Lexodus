use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::cases::*;
use crate::pages::user_management::*;

use crate::pages::cases::{
    create::CaseManagement, create::CreateCaseForm, edit_case::EditCase, list_case::ListCases,
    search_case::SearchCases, view_case::ViewCase,
};

// auth
use crate::pages::home::Home;
use crate::presentation::routes::login::Login;
use crate::presentation::routes::logout::Logout;
use crate::presentation::routes::signup::Signup;
use crate::providers::auth::{provide_auth, AuthContext};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth();
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    view! {
      <Stylesheet id="leptos" href="/pkg/lexodus.css"/>

      <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600&family=Merriweather:wght@700&display=swap" rel="stylesheet" />
      <Link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet" />


      // content for this welcome page
      <Router>
        <main>
          <Routes>
          <Route path="/" view=Home/>
          <Route
            path="/login"
            view=move || {
                view! { <Login action=auth_context.login/> }
            }
          />
          <Route path="cases" view=CaseManagement>
              <Route path="" view=ListCases/>
              <Route path="new" view=CreateCaseForm/>
              <Route path=":id" view=ViewCase/>
              <Route path=":id/edit" view=EditCase/>
              <Route path="search" view=SearchCases/>
          </Route>
            // <Route path="/case-management/activity" view=Activity/>
            <Route path="/users" view=UserManagement/>
            <Route path="/*any" view=NotFound/>
            <Route
              path="signup"
              view=move || {
                  view! { <Signup action=auth_context.signup/> }
              }
            />

            <Route
              path="login"
              view=move || {
                  view! { <Login action=auth_context.login/> }
              }
            />

            <Route
              path="logout"
              view=move || {
                  view! { <Logout action=auth_context.logout/> }
              }
            />

          </Routes>
        </main>
      </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_spin::ResponseOptions>();
        resp.set_status(404);
    }

    view! { <h1>"Not Found"</h1> }
}
