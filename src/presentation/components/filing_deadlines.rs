use leptos::*;

#[derive(Clone)]
struct FilingDeadline {
    case_name: String,
    case_number: String,
    deadline: String,
    description: String,
    status: String,
}

#[island]
pub fn FilingDeadlines() -> impl IntoView {
    let mock_deadlines = vec![
        FilingDeadline {
            case_name: "Smith v. Johnson".to_string(),
            case_number: "2023-CV-1234".to_string(),
            deadline: "2023-10-15".to_string(),
            description: "Response to Motion for Summary Judgment".to_string(),
            status: "Pending".to_string(),
        },
        FilingDeadline {
            case_name: "Doe v. Corp Inc.".to_string(),
            case_number: "2023-CV-5678".to_string(),
            deadline: "2023-10-20".to_string(),
            description: "Expert Witness Disclosure".to_string(),
            status: "Upcoming".to_string(),
        },
        FilingDeadline {
            case_name: "State v. Williams".to_string(),
            case_number: "2023-CR-9876".to_string(),
            deadline: "2023-10-10".to_string(),
            description: "Pre-Trial Motions".to_string(),
            status: "Overdue".to_string(),
        },
    ];

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
                    {mock_deadlines.into_iter().map(|deadline| view! {
                        <tr class="hover:bg-cyan-100 hover:text-gray-900">
                            <td class="border-t border-gray-700 px-4 py-2">{deadline.case_name}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{deadline.case_number}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{deadline.deadline}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{deadline.description}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{deadline.status}</td>
                        </tr>
                    }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
