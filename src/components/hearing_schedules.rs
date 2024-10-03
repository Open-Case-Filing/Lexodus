use crate::services::get_hearing_schedules::get_hearing_schedules;
use leptos::*;

#[component]
pub fn HearingSchedules() -> impl IntoView {
    let schedules = create_resource(|| (), |_| async move { get_hearing_schedules().await });
    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Hearing Schedules"</h3>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Name"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Number"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Date"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Time"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Courtroom"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || schedules.get().map(|result| match result {
                        Ok(schedules) => schedules.into_iter().map(|schedule| view! {
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">{schedule.case_name}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{schedule.case_number}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{schedule.date}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{schedule.time}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{schedule.courtroom}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{schedule.status}</td>
                            </tr>
                        }).collect_view(),
                        Err(_) => view! { <tr><td colspan="6" class="text-center">"Error loading hearing schedules"</td></tr> }.into_view()
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
