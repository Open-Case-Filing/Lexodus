use crate::services::get_action::get_action_menu_data;
use leptos::*;

#[component]
pub fn Action_Menu() -> impl IntoView {
    let data = create_resource(|| (), |_| async move { get_action_menu_data().await });

    view! {
        <div class="w-64 h-full bg-gray-800 p-4 overflow-y-auto flex-shrink-0 border-l border-r border-gray-700">
            <div class="space-y-4">
                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Upcoming Hearings"</h2>
                    <Transition fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                        <Suspense fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                            <ul class="mt-4 text-gray-400">
                                {move || data.get().map(|result| match result {
                                    Ok(data) => view! {
                                        <For
                                            each=move || data.upcoming_hearings.clone()
                                            key=|hearing| hearing.case_id.clone()
                                            children=move |hearing| {
                                                view! {
                                                    <li class="mb-2 hover:text-white transition-colors duration-300">
                                                        {format!("Case {}: {}", hearing.case_id, hearing.description)}
                                                    </li>
                                                }
                                            }
                                        />
                                    }.into_view(),
                                    Err(e) => view! { <li class="text-red-500">"Error loading hearings: "{e.to_string()}</li> }.into_view()
                                })}
                            </ul>
                        </Suspense>
                    </Transition>
                </div>

                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Pending Motions"</h2>
                    <Transition fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                        <Suspense fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                            <ul class="mt-4 text-gray-400">
                                {move || data.get().map(|result| match result {
                                    Ok(data) => view! {
                                        <For
                                            each=move || data.pending_motions.clone()
                                            key=|motion| motion.case_id.clone()
                                            children=move |motion| {
                                                view! {
                                                    <li class="mb-2 hover:text-white transition-colors duration-300">
                                                        {format!("Case {}: {}", motion.case_id, motion.description)}
                                                    </li>
                                                }
                                            }
                                        />
                                    }.into_view(),
                                    Err(e) => view! { <li class="text-red-500">"Error loading motions: "{e.to_string()}</li> }.into_view()
                                })}
                            </ul>
                        </Suspense>
                    </Transition>
                </div>

                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Recent Filings"</h2>
                    <Transition fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                        <Suspense fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                            <ul class="mt-4 text-gray-400">
                                {move || data.get().map(|result| match result {
                                    Ok(data) => view! {
                                        <For
                                            each=move || data.recent_filings.clone()
                                            key=|filing| filing.case_id.clone()
                                            children=move |filing| {
                                                view! {
                                                    <li class="mb-2 hover:text-white transition-colors duration-300">
                                                        {format!("Case {}: {}", filing.case_id, filing.description)}
                                                    </li>
                                                }
                                            }
                                        />
                                    }.into_view(),
                                    Err(e) => view! { <li class="text-red-500">"Error loading filings: "{e.to_string()}</li> }.into_view()
                                })}
                            </ul>
                        </Suspense>
                    </Transition>
                </div>

                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Courtroom Assignments"</h2>
                    <Transition fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                        <Suspense fallback=move || view! { <p class="text-gray-500 mt-2">"Loading..."</p> }>
                            <ul class="mt-4 text-gray-400">
                                {move || data.get().map(|result| match result {
                                    Ok(data) => view! {
                                        <For
                                            each=move || data.courtroom_assignments.clone()
                                            key=|assignment| assignment.case_id.clone()
                                            children=move |assignment| {
                                                view! {
                                                    <li class="mb-2 hover:text-white transition-colors duration-300">
                                                        {format!("Case {}: {}", assignment.case_id, assignment.courtroom)}
                                                    </li>
                                                }
                                            }
                                        />
                                    }.into_view(),
                                    Err(e) => view! { <li class="text-red-500">"Error loading assignments: "{e.to_string()}</li> }.into_view()
                                })}
                            </ul>
                        </Suspense>
                    </Transition>
                </div>
            </div>
        </div>
    }
}
