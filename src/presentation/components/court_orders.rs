use crate::services::get_court_orders::get_court_orders;
use leptos::*;

#[island]
pub fn CourtOrders() -> impl IntoView {
    let orders = create_resource(|| (), |_| async move { get_court_orders().await });

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
                    {move || orders.get().map(|result| match result {
                        Ok(orders) => orders.into_iter().map(|order| view! {
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">{order.order_number}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{order.case_name}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{order.date}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{order.order_details}</td>
                                <td class="border-t border-gray-700 px-4 py-2">{order.status}</td>
                            </tr>
                        }).collect_view(),
                        Err(_) => view! { <tr><td colspan="5" class="text-center">"Error loading court orders"</td></tr> }.into_view()
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
