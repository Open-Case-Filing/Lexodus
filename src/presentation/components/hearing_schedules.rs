use leptos::*;

#[derive(Clone)]
struct HearingSchedule {
    case_name: String,
    case_number: String,
    date: String,
    time: String,
    courtroom: String,
    status: String,
}

#[island]
pub fn HearingSchedules() -> impl IntoView {
    let mock_schedules = vec![
        HearingSchedule {
            case_name: "Smith v. Johnson".to_string(),
            case_number: "2023-CV-1234".to_string(),
            date: "2023-10-15".to_string(),
            time: "09:00 AM".to_string(),
            courtroom: "Courtroom 3A".to_string(),
            status: "Scheduled".to_string(),
        },
        HearingSchedule {
            case_name: "Doe v. Corp Inc.".to_string(),
            case_number: "2023-CV-5678".to_string(),
            date: "2023-10-20".to_string(),
            time: "02:30 PM".to_string(),
            courtroom: "Courtroom 5B".to_string(),
            status: "Rescheduled".to_string(),
        },
        HearingSchedule {
            case_name: "State v. Williams".to_string(),
            case_number: "2023-CR-9876".to_string(),
            date: "2023-10-18".to_string(),
            time: "10:45 AM".to_string(),
            courtroom: "Courtroom 2C".to_string(),
            status: "In Progress".to_string(),
        },
    ];

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
                    {mock_schedules.into_iter().map(|schedule| view! {
                        <tr class="hover:bg-cyan-100 hover:text-gray-900">
                            <td class="border-t border-gray-700 px-4 py-2">{schedule.case_name}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{schedule.case_number}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{schedule.date}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{schedule.time}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{schedule.courtroom}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{schedule.status}</td>
                        </tr>
                    }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
