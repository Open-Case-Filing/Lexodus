use leptos::*;

#[derive(Clone)]
struct CourtOrder {
    order_number: String,
    case_name: String,
    date: String,
    order_details: String,
    status: String,
}

#[island]
pub fn CourtOrders() -> impl IntoView {
    let mock_orders = vec![
        CourtOrder {
            order_number: "CO-2023-001".to_string(),
            case_name: "Smith v. Johnson".to_string(),
            date: "2023-09-15".to_string(),
            order_details: "Motion to Dismiss Granted".to_string(),
            status: "Executed".to_string(),
        },
        CourtOrder {
            order_number: "CO-2023-002".to_string(),
            case_name: "Doe v. Corp Inc.".to_string(),
            date: "2023-09-14".to_string(),
            order_details: "Preliminary Injunction Issued".to_string(),
            status: "Pending".to_string(),
        },
        CourtOrder {
            order_number: "CO-2023-003".to_string(),
            case_name: "State v. Williams".to_string(),
            date: "2023-09-13".to_string(),
            order_details: "Evidence Suppression Order".to_string(),
            status: "Under Review".to_string(),
        },
    ];

    view! {
        <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mt-4">
            <h3 class="text-lg font-semibold mb-4 text-gray-300">"Court Orders"</h3>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 hover:table-fixed">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-gray-400">"Order Number"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Case Name"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Date"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Order Details"</th>
                            <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                    {mock_orders.into_iter().map(|order| view! {
                        <tr class="hover:bg-cyan-100 hover:text-gray-900">
                            <td class="border-t border-gray-700 px-4 py-2">{order.order_number}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{order.case_name}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{order.date}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{order.order_details}</td>
                            <td class="border-t border-gray-700 px-4 py-2">{order.status}</td>
                        </tr>
                    }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
