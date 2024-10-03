use leptos::*;
use crate::layouts::wide::Wide_Layout;

#[component]
pub fn Changelog() -> impl IntoView {
    view! {
        <Wide_Layout>
            <div class="bg-gray-800 p-6 rounded-lg outline outline-offset-2 outline-cyan-500 mx-4 my-8">
                <h3 class="text-lg font-semibold mb-4 text-gray-300">"Changelog"</h3>
                <div class="overflow-x-auto">
                    <table class="min-w-full bg-gray-800 hover:table-fixed">
                        <thead>
                            <tr>
                                <th class="px-4 py-2 text-left text-gray-400">"Version"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Date"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Description"</th>
                                <th class="px-4 py-2 text-left text-gray-400">"Status"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">"1.0.1"</td>
                                <td class="border-t border-gray-700 px-4 py-2">"July 10, 2024"</td>
                                <td class="border-t border-gray-700 px-4 py-2">"Fixed bug in user authentication flow. Improved performance of data fetching. Updated UI for better accessibility."</td>
                                <td class="border-t border-gray-700 px-4 py-2 text-right">"Released"</td>
                            </tr>
                            <tr class="hover:bg-cyan-100 hover:text-gray-900">
                                <td class="border-t border-gray-700 px-4 py-2">"1.0.0"</td>
                                <td class="border-t border-gray-700 px-4 py-2">"June 30, 2024"</td>
                                <td class="border-t border-gray-700 px-4 py-2">"Initial release with core features. Implemented user management and authentication. Added support for case management."</td>
                                <td class="border-t border-gray-700 px-4 py-2 text-right">"Released"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </Wide_Layout>
    }
}
