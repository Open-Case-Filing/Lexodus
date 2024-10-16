use crate::functions::{self,auth::Login};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::providers::auth::AuthContext;
#[component]
pub fn Login(action: Action<functions::auth::Login, Result<(), ServerFnError>>) -> impl IntoView {
  let login_user = create_server_action::<Login>();
  let _login_value = login_user.value();
  let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    view! {
      <Meta property="og:title" content="Login"/>
      <Title text="Login"/>
      <Meta name="description" content="Login to the site"/>
      <Meta property="og:description" content="Login to the site"/>


        <div class="bg-gradient-to-r from-lexodus-800 to-blue-900 flex items-center justify-center min-h-screen flex-col">
        <div class="text-center mb-8">
            <h1 class="text-4xl font-extrabold text-white mb-2">"Lexodus"</h1>
            <p class="text-lg text-gray-300">"Enhancing judicial efficiency through technology"</p>
        </div>
        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg shadow-lg w-full max-w-sm">
            <h2 class="text-2xl font-bold mb-6 text-center text-white">"Login"</h2>
          <ActionForm action=action class="space-y-6">
                <div class="mb-4">
                    <label for="username" class="block text-white text-sm font-bold mb-2">"Username"</label>
                    <input type="username" id="username" name="username" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"  aria-describedby="username-error" required/>
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-white text-sm font-bold mb-2">"Password"</label>
                    <input type="password" id="password" name="password" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" aria-describedby="password-error" autoComplete="current-password" required/>
                </div>

                <div class="flex items-center justify-between">
                    <button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                        "Log in"
                    </button>
                </div>
                           <div class="flex items-center justify-between">
              <div class="flex items-center">
                <input
                  id="remember"
                  name="remember"
                  type="checkbox"
                  class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <label
                  for="remember"
                  class="ml-2 block text-sm text-gray-900 dark:text-white"
                >
                  "Remember me"
                </label>
              </div>
              <div class="text-center text-sm text-gray-500">
                "Don't have an account?"
              </div>
                <a rel="external" class="text-blue-500 underline" href="/signup">
                  "Sign up"
                </a>
                <a rel="external" class="text-blue-500 underline" href="/cases">
                  "Already logged in?"
                </a>


              <Transition fallback=move || ()>
                {move || {
                    let user = move || match auth_context.user.get() {
                        Some(Ok(Some(user))) => Some(user),
                        Some(Ok(None)) => None,
                        Some(Err(_)) => None,
                        None => None,
                    };
                    view! {

                      <Show
                        when=move || user().is_some()
                      >
                        <Redirect path="/cases" />
                      </Show>



                    }
                }}

              </Transition>
            </div>
             </ActionForm>


        </div>
    </div>
    }
}
