use leptos::*;
use leptos_meta::*;
use crate::layouts::default::*;
use crate::components::search_bar::*;


#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Dashboard | Open Case Filing System"/>
        <Title text="Dashboard | Open Case Filing System"/>
        <Meta name="description" content="Dashboard overview for OCFS with real time case numbers for the week, month, year."/>
        <Meta property="og:description" content="A dashboard with case management statistics and ability to see civil and criminal case information."/>
        <Meta
          property="og:image"
          content="https://en.wikipedia.org/wiki/CM/ECF#/media/File:CM_ECF_logo.png"
        />
        <Default_Layout>
            <div class="flex-1 p-8 overflow-auto">
                <div class="max-w-7xl mx-auto">
                    <div class="flex justify-between items-center mb-8">
                        <h2 class="text-2xl font-semibold">Welcome to Open Case Filing System, Tyler</h2>
                        <div class="relative">
                            <select aria-label="filter by interval" class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg text-white p-2 rounded focus:outline-none">
                                <option>"Last week"</option>
                                <option>"Last month"</option>
                                <option>"Last year"</option>
                            </select>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg">
                            <p class="text-gray-300 text-sm">"Total cases"</p>
                            <p class="text-2xl font-bold">"1,024"</p>
                            <p class="text-green-400 text-sm mt-1">"+3.2% from last week"</p>
                        </div>
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg">
                            <p class="text-gray-300 text-sm">"Active cases"</p>
                            <p class="text-2xl font-bold">"512"</p>
                            <p class="text-red-400 text-sm mt-1">"-1.8% from last week"</p>
                        </div>
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg">
                            <p class="text-gray-300 text-sm">"Closed cases"</p>
                            <p class="text-2xl font-bold">"256"</p>
                            <p class="text-green-400 text-sm mt-1">"+2.7% from last week"</p>
                        </div>
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg">
                            <p class="text-gray-300 text-sm">"Pending cases"</p>
                            <p class="text-2xl font-bold">"256"</p>
                            <p class="text-green-400 text-sm mt-1">"+5.1% from last week"</p>
                        </div>
                    </div>

                    <div class="mb-8">
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-4 rounded-lg shadow-lg">
                            <nav class="flex flex-wrap justify-center space-x-4 text-white text-sm">
                                <a href="/participants" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Recent Case Activity"</a>
                                <span>"|"</span>
                                <a href="/dashboard/overview" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Court Orders"</a>
                                <span>"|"</span>
                                <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Filing Deadlines"</a>
                                <span>"|"</span>
                                <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Hearing Schedules"</a>
                                <span>"|"</span>
                                <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Case Notifications"</a>
                                <span>"|"</span>
                                <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Attorney Notes"</a>
                                <span>"|"</span>
                                <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Document Uploads"</a>
                                <span>"|"</span>
                                <a href="#" class="hover:bg-blue-500 px-2 py-1 rounded transition duration-200">"Motions and Filings"</a>
                            </nav>
                        </div>
                    </div>

                    <SearchBar />
                </div>
            </div>
        </Default_Layout>
    }
}
