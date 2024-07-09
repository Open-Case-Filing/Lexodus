// use crate::models::case::Case;
use crate::services::case_service::search_cases;
use leptos::*;

#[island]
pub fn SearchBar() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let cases = create_resource(move || search_query.get(), move |query| search_cases(query));

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
                <h3 class="text-lg font-semibold mb-4">"Recent Case Activity"</h3>
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
                        {move || cases.get().map(|result| match result {
                            Ok(cases) => cases.into_iter().map(|case| view! {
                                <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                    <td class="border-t border-gray-700 px-4 py-2">{case.case_name}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.case_number}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.court}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.date_filed.format("%Y-%m-%d").to_string()}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.assigned_to.unwrap_or_default()}</td>
                                    <td class="border-t border-gray-700 px-4 py-2">{case.status}</td>
                                </tr>
                            }).collect_view(),
                            Err(_) => view! { <tr><td colspan="6" class="text-center">"Error loading cases"</td></tr> }.into_view()
                        })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
