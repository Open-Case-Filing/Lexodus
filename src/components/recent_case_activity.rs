use crate::services::get_recent_case_activities::get_recent_case_activities;
use leptos::*;

#[component]
pub fn RecentCaseActivity() -> impl IntoView {
    let activities = create_resource(|| (), |_| async move { get_recent_case_activities().await });

    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Recent Case Activity"</h3>
            <div class="overflow-x-auto">
                <table class="min-wa-full bg-gray-800 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Name"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Number"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Activity"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Date"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Involved Parties"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || activities.get().map(|result| match result {
                        Ok(activities) => activities.into_iter().map(|activity| view! {
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">{activity.case_name}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{activity.case_number}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{activity.activity}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{activity.date}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{activity.involved_parties}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{activity.status}</td>
                            </tr>
                        }).collect_view(),
                        Err(_) => view! { <tr><td colspan="6" class="text-center">"Error loading case activities"</td></tr> }.into_view()
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
