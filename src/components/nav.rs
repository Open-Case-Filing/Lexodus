use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <aside id="nav" class="w-full lg:w-64 bg-gray-800 h-full text-white min-w-[250px]">
            <div class="p-4 text-center">
                <h2 href="/" class="text-2xl font-semibold ">Home</h2>
            </div>
            <div class="p-4 rounded-lg shadow-lg w-full max-w-4xl mx-auto">
                <nav class="flex flex-col space-y-4 text-white text-sm">
                    <NavLink url="/dashboard/overview" icon_type="dashboard" label="Dashboard" />
                    <NavLink url="/case/add" icon_type="cases" label="Cases" />
                    <NavLink url="/case/activity" icon_type="cases" label="Case Activity" />
                    <NavLink url="/user-management/create" icon_type="signup" label="Sign up" />
                    // <NavLink url="/documents/activity" icon_type="documents" label="Documents" />
                    // <NavLink url="/scheduling/activity" icon_type="scheduling" label="Scheduling" />
                    // <NavLink url="/reports/activity" icon_type="reports" label="Reports & Analytics" />
                    // <NavLink url="/user-management" icon_type="user_management" label="User Management" />
                    // <NavLink url="/support" icon_type="support" label="Support" />
                    // <NavLink url="/changelog" icon_type="changelog" label="Changelog" />
                    // <NavLink url="/hearings" icon_type="calendar" label="Upcoming Hearings" />
                    // <NavLink url="/motions" icon_type="gavel" label="Pending Motions" />
                    // <NavLink url="/filings" icon_type="documents" label="Recent Filings" />
                    // <NavLink url="/assignments" icon_type="courthouse" label="Courtroom Assignments" />
                    // <NavLink url="/judges" icon_type="judge" label="Judges" />
                    // <div class="mt-8 flex items-center">
                    //     <img src="https://via.placeholder.com/40" class="w-10 h-10 rounded-full mr-4" alt="User profile picture" />
                    //     <div>
                    //         <p class="text-sm font-semibold">"Tyler"</p>
                    //         <p class="text-xs text-gray-400">"Tyler@example.com"</p>
                    //     </div>
                    // </div>
                </nav>
            </div>
        </aside>
    }
}

#[component]
pub fn NavLink(url: &'static str, icon_type: &'static str, label: &'static str) -> impl IntoView {
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
        _ => "",
    };
    view! {
        <a rel="external" href={url} class="flex items-center text-gray-400 hover:text-white">
            <span inner_html={icon_svg}></span>
            {label}
        </a>
    }
}
