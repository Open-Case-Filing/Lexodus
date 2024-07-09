use crate::services::get_action::get_action_menu_data;
use leptos::*;

#[island]
pub fn Action_Menu() -> impl IntoView {
    let data = create_resource(|| (), |_| async move { get_action_menu_data().await });

    view! {
        <div class="w-full md:w-1/4 lg:w-1/5 bg-gray-800 p-4 hidden md:block min-w-[250px]" id="sidebar">
            <div class="mt-8 space-y-6 mr-2">
                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Upcoming Hearings"</h2>
                    <ul class="mt-4 text-gray-400">
                        {move || data.get().map(|result| match result {
                            Ok(data) => data.upcoming_hearings.into_iter().map(|hearing| view! {
                                <li class="mb-2 hover:text-white transition-colors duration-300">{format!("Case {}: {}", hearing.case_id, hearing.description)}</li>
                            }).collect_view(),
                            Err(_) => view! { <li class="text-red-500">"Error loading hearings"</li> }.into_view()
                        })}
                    </ul>
                </div>

                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Pending Motions"</h2>
                    <ul class="mt-4 text-gray-400">
                        {move || data.get().map(|result| match result {
                            Ok(data) => data.pending_motions.into_iter().map(|motion| view! {
                                <li class="mb-2 hover:text-white transition-colors duration-300">{format!("Case {}: {}", motion.case_id, motion.description)}</li>
                            }).collect_view(),
                            Err(_) => view! { <li class="text-red-500">"Error loading motions"</li> }.into_view()
                        })}
                    </ul>
                </div>

                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Recent Filings"</h2>
                    <ul class="mt-4 text-gray-400">
                        {move || data.get().map(|result| match result {
                            Ok(data) => data.recent_filings.into_iter().map(|filing| view! {
                                <li class="mb-2 hover:text-white transition-colors duration-300">{format!("Case {}: {}", filing.case_id, filing.description)}</li>
                            }).collect_view(),
                            Err(_) => view! { <li class="text-red-500">"Error loading filings"</li> }.into_view()
                        })}
                    </ul>
                </div>

                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Courtroom Assignments"</h2>
                    <ul class="mt-4 text-gray-400">
                        {move || data.get().map(|result| match result {
                            Ok(data) => data.courtroom_assignments.into_iter().map(|assignment| view! {
                                <li class="mb-2 hover:text-white transition-colors duration-300">{format!("Case {}: {}", assignment.case_id, assignment.courtroom)}</li>
                            }).collect_view(),
                            Err(_) => view! { <li class="text-red-500">"Error loading assignments"</li> }.into_view()
                        })}
                    </ul>
                </div>
            </div>
        </div>
    }
}
