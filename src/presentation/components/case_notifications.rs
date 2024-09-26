use leptos::*;

#[derive(Clone)]
struct Notification {
    notification_id: String,
    case_name: String,
    date: String,
    message: String,
    status: String,
}

#[island]
pub fn CaseNotifications() -> impl IntoView {
    let mock_notifications = vec![
        Notification {
            notification_id: "1".to_string(),
            case_name: "Smith v. Johnson".to_string(),
            date: "2023-09-15".to_string(),
            message: "New document filed".to_string(),
            status: "Unread".to_string(),
        },
        Notification {
            notification_id: "2".to_string(),
            case_name: "Doe v. Corp Inc.".to_string(),
            date: "2023-09-14".to_string(),
            message: "Hearing scheduled".to_string(),
            status: "Read".to_string(),
        },
        Notification {
            notification_id: "3".to_string(),
            case_name: "State v. Williams".to_string(),
            date: "2023-09-13".to_string(),
            message: "Motion filed".to_string(),
            status: "Unread".to_string(),
        },
    ];

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
                    {mock_notifications.into_iter().map(|notification| view! {
                        <tr class="hover:bg-cyan-100 hover:text-gray-900">
                            <td class="border-t border-gray-700 px-4 py-2">{notification.notification_id}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{notification.case_name}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{notification.date}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{notification.message}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{notification.status}</td>
                        </tr>
                    }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
