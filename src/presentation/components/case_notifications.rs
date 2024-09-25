use crate::services::get_case_notifications::get_case_notifications;
use leptos::*;

#[island]
pub fn CaseNotifications() -> impl IntoView {
    let notifications = create_resource(|| (), |_| async move { get_case_notifications().await });

    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Case Notifications"</h3>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"Notification ID"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Name"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Date"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Message"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || notifications.get().map(|result| match result {
                        Ok(notifications) => notifications.into_iter().map(|notification| view! {
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">{notification.notification_id}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{notification.case_name}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{notification.date}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{notification.message}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{notification.status}</td>
                            </tr>
                        }).collect_view(),
                        Err(_) => view! { <tr><td colspan="5" class="text-center">"Error loading case notifications"</td></tr> }.into_view()
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
