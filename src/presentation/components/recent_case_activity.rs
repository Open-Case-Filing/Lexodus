use leptos::*;

#[derive(Clone)]
struct CaseActivity {
    case_name: String,
    case_number: String,
    activity: String,
    date: String,
    involved_parties: String,
    status: String,
}

#[island]
pub fn RecentCaseActivity() -> impl IntoView {
    let mock_activities = vec![
        CaseActivity {
            case_name: "Smith v. Johnson".to_string(),
            case_number: "2023-CV-1234".to_string(),
            activity: "Motion Filed".to_string(),
            date: "2023-09-25".to_string(),
            involved_parties: "Plaintiff's Counsel".to_string(),
            status: "Pending Review".to_string(),
        },
        CaseActivity {
            case_name: "Doe v. Corp Inc.".to_string(),
            case_number: "2023-CV-5678".to_string(),
            activity: "Hearing Scheduled".to_string(),
            date: "2023-09-24".to_string(),
            involved_parties: "All Parties".to_string(),
            status: "Scheduled".to_string(),
        },
        CaseActivity {
            case_name: "State v. Williams".to_string(),
            case_number: "2023-CR-9876".to_string(),
            activity: "Evidence Submitted".to_string(),
            date: "2023-09-23".to_string(),
            involved_parties: "Prosecution".to_string(),
            status: "Under Review".to_string(),
        },
    ];

    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Recent Case Activity"</h3>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 hover:table-fixed">
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
                    {mock_activities.into_iter().map(|activity| view! {
                        <tr class="hover:bg-cyan-100 hover:text-gray-900">
                            <td class="border-t border-gray-700 px-4 py-2">{activity.case_name}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{activity.case_number}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{activity.activity}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{activity.date}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{activity.involved_parties}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{activity.status}</td>
                        </tr>
                    }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
