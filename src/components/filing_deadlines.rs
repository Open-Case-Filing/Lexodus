use crate::services::get_filing_deadlines::get_filing_deadlines;
use leptos::*;

#[component]
pub fn FilingDeadlines() -> impl IntoView {
    let deadlines = create_resource(|| (), |_| async move { get_filing_deadlines().await });
    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Filing Deadlines"</h3>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Name"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Number"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Deadline"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Description"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || deadlines.get().map(|result| match result {
                        Ok(deadlines) => deadlines.into_iter().map(|deadline| view! {
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">{deadline.case_name}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{deadline.case_number}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{deadline.deadline}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{deadline.description}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{deadline.status}</td>
                            </tr>
                        }).collect_view(),
                        Err(_) => view! { <tr><td colspan="5" class="text-center">"Error loading filing deadlines"</td></tr> }.into_view()
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
