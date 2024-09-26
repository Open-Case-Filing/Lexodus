use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Case {
    title: String,
    case_number: String,
    court_id: String,
    filed_date: String,
    judge_id: Option<i64>,
    status: String,
}

fn mock_cases() -> Vec<Case> {
    vec![
        Case {
            title: "Smith v. Johnson".to_string(),
            case_number: "2023-CV-1234".to_string(),
            court_id: "District Court".to_string(),
            filed_date: "2023-09-15".to_string(),
            judge_id: Some(101),
            status: "Active".to_string(),
        },
        Case {
            title: "Doe v. Corp Inc.".to_string(),
            case_number: "2023-CV-5678".to_string(),
            court_id: "Superior Court".to_string(),
            filed_date: "2023-08-20".to_string(),
            judge_id: Some(102),
            status: "Pending".to_string(),
        },
        Case {
            title: "State v. Williams".to_string(),
            case_number: "2023-CR-9876".to_string(),
            court_id: "Criminal Court".to_string(),
            filed_date: "2023-07-10".to_string(),
            judge_id: None,
            status: "Closed".to_string(),
        },
    ]
}

fn search_cases(query: &str) -> Vec<Case> {
    let cases = mock_cases();
    cases
        .into_iter()
        .filter(|case| {
            case.title.to_lowercase().contains(&query.to_lowercase())
                || case.case_number.to_lowercase().contains(&query.to_lowercase())
                || case.status.to_lowercase().contains(&query.to_lowercase())
        })
        .collect()
}

#[island]
pub fn SearchBar() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let cases = create_memo(move |_| search_cases(&search_query.get()));

    view! {
        <div>
            <div class="mb-8 w-full">
                <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg rounded-lg shadow-lg w-full max-w-4xl mx-auto outline outline-offset-2 outline-cyan-500">
                    <nav class="flex flex-wrap justify-center space-x-4 text-white text-sm">
                        <input
                            id="search"
                            type="text"
                            class="w-full bg-gray-800 text-white p-2 rounded focus:outline-none"
                            placeholder="Search for cases..."
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        />
                    </nav>
                </div>
            </div>
            // Display search results
            <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
                <h3 class="text-lg font-semibold mb-4">"Search Results"</h3>
                <div class="overflow-x-auto">
                    <table class="min-w-full bg-gray-800 hover:table-fixed">
                        <thead>
                            <tr>
                                <th class="px-4 py-2 text-left text-gray-400">"Case Name"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Case Number"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Court"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Date Filed"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Assigned To"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                            </tr>
                        </thead>
                        <tbody>
                        {move || cases.get().into_iter().map(|case| view! {
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">{case.title}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{case.case_number}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{case.court_id}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{case.filed_date}</td>
                                <td class="border-t border-gray-700 px-4 py-2">
                                    {case.judge_id.map_or_else(|| "-".to_string(), |id| id.to_string())}
                                </td>
                                <td class="border-t border-gray-700 px-4 py-2">{case.status}</td>
                            </tr>
                        }).collect_view()}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
