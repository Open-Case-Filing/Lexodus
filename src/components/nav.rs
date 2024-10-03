use leptos::*;
use crate::functions::save_count::*;
use crate::providers::auth::AuthContext;


#[component]
pub fn Nav() -> impl IntoView {
      let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    view! {
        <aside id="nav" class="w-full lg:w-64 bg-gray-800 h-full text-white max-w-[48px]">
            <div class="rounded-lg shadow-lg w-full max-w-4xl mx-auto">
                <nav class="flex flex-col space-y-4 text-white text-sm">
                    <NavLink url="/dashboard/overview" icon_type="dashboard" label="Dashboard" />
                    <NavLink url="/case-management" icon_type="cases" label="Case Management" />
                    // <NavLink url="/case-management/activity" icon_type="cases" label="Case Activity" />
                    <NavLink url="/user-management" icon_type="user_management" label="User Management" />
        <Transition fallback=move || ()>
          {move || {
              let user = move || match auth_context.user.get() {
                  Some(Ok(Some(user))) => Some(user),
                  Some(Ok(None)) => None,
                  Some(Err(_)) => None,
                  None => None,
              };
              view! {
                // logging::log!("USER: {:#?}", user());
                <Show
                  when=move || user().is_some()
                  fallback=|| {
                      view! {
                        <NavLink url="/login" icon_type="logout" label="logout" />

                      }
                  }
                >

                <NavLink url="/logout" icon_type="logout" label="logout" />

                </Show>
              }
          }}

        </Transition>
                </nav>
            </div>
        </aside>
    }
}

#[component]
pub fn NavLink(url: &'static str, icon_type: &'static str, label: &'static str) -> impl IntoView {

  let (count, set_count) = create_signal(0);
  let on_click = move |_| {
      set_count.update(|count| *count += 1);
      spawn_local(async move {

          save_count(count.get()).await.unwrap();
      });
  };


    let icon_svg = match icon_type {
        "calendar" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-8 4h8m-8 0H5a2 2 0 00-2 2v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3M8 7h8" />
            </svg>
        "#
        }
        "gavel" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2l2 7h7l-6 5 2 7-6-5-6 5 2-7-6-5h7l2-7z" />
            </svg>
        "#
        }
        "documents" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h10V5H7v2zm0 2h10v10H7V9zm1-4h8V3H8v2zm-3 2v12a2 2 0 002 2h10a2 2 0 002-2V7H5zm4 4h2v6H9v-6zm4 0h2v6h-2v-6z" />
            </svg>
        "#
        }
        "courthouse" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2L2 7v2h20V7L12 2zM2 10v2h20v-2H2zm0 4v8h6v-6h8v6h6v-8H2z" />
            </svg>
        "#
        }
        "dashboard" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h18a2 2 0 012 2v16a2 2 0 01-2 2H3a2 2 0 01-2-2V5a2 2 0 012-2z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3v18m18-18v18M3 10h18M3 14h18"></path>
            </svg>
        "#
        }
        "cases" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h10M7 16h10M5 6v14m14-14v14M3 4h18"></path>
            </svg>
        "#
        }
        "scheduling" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v1M8 4v1M16 4v1M12 15v1M8 15v1M16 15v1M4 8h16M4 11h16M4 14h16"></path>
            </svg>
        "#
        }
        "reports" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4v16h18V4H3zm9 14H8v-4h4v4zm0-6H8V8h4v4zm6 6h-4v-4h4v4zm0-6h-4V8h4v4z"></path>
            </svg>
        "#
        }
        "user_management" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7h18M5 10v10h14V10M9 15h6"></path>
            </svg>
        "#
        }
        "support" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 5.636a9 9 0 11-12.728 0A9 9 0 0118.364 5.636z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2v4M12 18v4M2 12h4m12 0h4M16.95 7.05l2.828 2.828M7.05 16.95l-2.828-2.828"></path>
            </svg>
        "#
        }
        "changelog" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7v4M5 7v4M7 7h4M13 7h4M9 21V8M15 21V8M3 3h18a2 2 0 012 2v16a2 2 0 01-2 2H3a2 2 0 01-2-2V5a2 2 0 012-2z"></path>
            </svg>
        "#
        }
        "judge" => {
            r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 2l2 7h7l-6 5 2 7-6-5-6 5 2-7-6-5h7l2-7z" />
            </svg>
            "#
        }
        "signup" => { r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <circle cx="12" cy="7" r="4" />
                <path d="M12 11v6" />
                <path d="M9 17h6" />
                <path d="M3 21h18" />
            </svg>
        "# },
        "login" => {
          r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" />
                <polyline points="10 17 15 12 10 7" />
                <line x1="15" y1="12" x2="3" y2="12" />
            </svg>
        "#
        },
        "logout" => r#"
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
                <polyline points="16 17 21 12 16 7" />
                <line x1="21" y1="12" x2="9" y2="12" />
            </svg>
        "#,
        _ => "",
    };
    view! {
        <a rel="external" on:click=on_click href={url} class="flex items-center text-gray-400 hover:text-white">
            <span inner_html={icon_svg}></span>
            // {label}
        </a>
    }
}
