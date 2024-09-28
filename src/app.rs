use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::cases::*;
use crate::pages::user_management::*;
use crate::pages::changelog::Changelog;
use crate::pages::dashboard::Dashboard;
use crate::pages::login::Login as Home;

// auth
use crate::providers::auth::{provide_auth, AuthContext};
use crate::presentation::routes::logout::Logout;
use crate::presentation::routes::login::Login;
use crate::presentation::routes::signup::Signup;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth();
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");


    view! {
      <Stylesheet id="leptos" href="/pkg/lexodus.css"/>

      // content for this welcome page
      <Router>
        <main>
          <Routes>
          <Route
            path="/"
            view=move || {
                view! { <Login action=auth_context.login/> }
            }
          />
            <Route path="/dashboard/overview" view=Dashboard/>
            <Route path="/case-management" view=CaseManagement/>
            <Route path="/changelog" view=Changelog/>
            // <Route path="/case-management/activity" view=Activity/>
            <Route path="/user-management" view=UserManagement/>
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
