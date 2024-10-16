use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use crate::layouts::default::DefaultLayout;

#[component]
pub fn SearchCases() -> impl IntoView {
    let search_cases = create_server_action::<SearchCasesAction>();
    let value = search_cases.value();

    view! {
        <DefaultLayout>
            <section class="bg-white p-6 rounded-lg shadow-lg border border-lexodus-200 mt-8 relative">
                <h3 class="text-xl font-semibold text-lexodus-800 mb-6">"Search Cases"</h3>

                <ActionForm action=search_cases>
                    // Case Number Input
                    <div class="mb-4">
                        <label for="case_number" class="block text-lexodus-700 mb-1">"Case Number:"</label>
                        <input type="text" id="case_number" name="case_number" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
                    </div>

                    // Case Status Radio Buttons
                    <div class="mb-4">
                        <label class="block text-lexodus-700 mb-1">"Case Status:"</label>
                        <div class="flex space-x-4">
                            <label class="inline-flex items-center">
                                <input type="radio" name="case_status" value="open" class="form-radio text-lexodus-600"/>
                                <span class="ml-2">"Open"</span>
                            </label>
                            <label class="inline-flex items-center">
                                <input type="radio" name="case_status" value="closed" class="form-radio text-lexodus-600"/>
                                <span class="ml-2">"Closed"</span>
                            </label>
                            <label class="inline-flex items-center">
                                <input type="radio" name="case_status" value="all" class="form-radio text-lexodus-600"/>
                                <span class="ml-2">"All"</span>
                            </label>
                        </div>
                    </div>

                    // Filed Date Inputs
                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2 mb-4">
                        <div>
                            <label for="filed_date_from" class="block text-lexodus-700 mb-1">"Filed Date (from):"</label>
                            <input type="date" id="filed_date_from" name="filed_date_from" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
                        </div>
                        <div>
                            <label for="filed_date_to" class="block text-lexodus-700 mb-1">"Filed Date (to):"</label>
                            <input type="date" id="filed_date_to" name="filed_date_to" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
                        </div>
                    </div>

                    // Cause of Action Select
                    <div class="mb-4">
                        <label for="cause_of_action" class="block text-lexodus-700 mb-1">"Cause of Action:"</label>
                        <select id="cause_of_action" name="cause_of_action" multiple class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500">
                            <option value="0">"0 (No cause code entered)"</option>
                            <option value="02:0431">"02:0431 (Federal Election Commission: Failure Enforce Compliance)"</option>
                            <option value="05:0552">"05:0552 (Freedom of Information Act)"</option>
                            // Add more options as needed
                        </select>
                    </div>

                    // Nature of Suit Select
                    <div class="mb-4">
                        <label for="nature_suit" class="block text-lexodus-700 mb-1">"Nature of Suit:"</label>
                        <select id="nature_suit" name="nature_suit" multiple class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500">
                            <option value="0">"0 (zero)"</option>
                            <option value="110">"110 (Insurance)"</option>
                            // Add more options as needed
                        </select>
                    </div>

                    // Last/Business Name Input
                    <div class="mb-4">
                        <label for="last_business_name" class="block text-lexodus-700 mb-1">"Last/Business Name:"</label>
                        <input type="text" id="last_business_name" name="last_business_name" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
                    </div>

                    // Exact Matches Checkbox
                    <div class="mb-4">
                        <label class="inline-flex items-center">
                            <input type="checkbox" id="exact_matches_only" name="exact_matches_only" class="form-checkbox text-lexodus-600"/>
                            <span class="ml-2">"Exact matches only"</span>
                        </label>
                    </div>

                    // First Name, Middle Name, and Type Inputs
                    <div class="grid grid-cols-1 gap-6 md:grid-cols-3 mb-4">
                        <div>
                            <label for="first_name" class="block text-lexodus-700 mb-1">"First Name:"</label>
                            <input type="text" id="first_name" name="first_name" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
                        </div>
                        <div>
                            <label for="middle_name" class="block text-lexodus-700 mb-1">"Middle Name:"</label>
                            <input type="text" id="middle_name" name="middle_name" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500"/>
                        </div>
                        <div>
                            <label for="type_field" class="block text-lexodus-700 mb-1">"Select Type:"</label>
                            <select id="type_field" name="type_field" class="w-full px-4 py-2 bg-gray-100 text-lexodus-800 rounded border border-lexodus-200 focus:outline-none focus:ring-2 focus:ring-lexouds-500">
                                <option value="attorney">"Attorney"</option>
                                <option value="party">"Party"</option>
                            </select>
                        </div>
                    </div>

                    // Submit and Clear Buttons
                    <div class="flex justify-end space-x-4">
                        <button type="submit" class="px-4 py-2 bg-lexodus-500 text-white rounded font-semibold hover:bg-lexodus-600 focus:outline-none focus:ring-2 focus:ring-lexodus-500">"Run Query"</button>
                        <button type="reset" class="px-4 py-2 bg-gray-500 text-white rounded font-semibold hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500">"Clear"</button>
                    </div>
                </ActionForm>

                // Loading State
                <Show
                    when=move || search_cases.pending().get()
                    fallback=|| view! { <div></div> }
                >
                    <div class="mt-4 text-lexodus-700">"Searching cases..."</div>
                </Show>

                // Results or Error Display
                {move || value.get().map(|result| match result {
                    Ok(cases) => view! {
                        <div class="mt-4">
                            <h4 class="text-lg font-semibold text-lexodus-800 mb-2">"Search Results"</h4>
                            // Implement a component to display the search results
                            <SearchResultsTable cases=cases />
                        </div>
                    },
                    Err(e) => view! {
                        <div class="mt-4 text-red-500">{e.to_string()}</div>
                    },
                })}
            </section>
        </DefaultLayout>
    }
}

#[server(SearchCasesAction, "/api/search_cases")]
pub async fn search_cases(
    case_number: Option<String>,
    case_status: Option<String>,
    filed_date_from: Option<String>,
    filed_date_to: Option<String>,
    cause_of_action: Option<Vec<String>>,
    nature_suit: Option<Vec<String>>,
    last_business_name: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    type_field: Option<String>,
    exact_matches_only: Option<bool>,
) -> Result<Vec<Case>, ServerFnError> {
    // Implement your server-side search logic here
    // This is a placeholder that returns an empty vector
    Ok(vec![])
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    // Define your Case struct fields here
    case_number: String,
    // Add other fields as needed
}

#[component]
fn SearchResultsTable(cases: Vec<Case>) -> impl IntoView {
    view! {
        <table class="w-full mt-4 border-collapse border border-lexodus-200">
            <thead>
                <tr class="bg-lexodus-100">
                    <th class="p-2 text-left">"Case Number"</th>
                    // Add more table headers as needed
                </tr>
            </thead>
            <tbody>
                {cases.into_iter().map(|case| view! {
                    <tr class="border-t border-lexodus-200">
                        <td class="p-2">{case.case_number}</td>
                        // Add more table cells as needed
                    </tr>
                }).collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}
