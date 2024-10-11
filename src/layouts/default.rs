use leptos::*;

#[component]
pub fn Default_Layout(children: Children) -> impl IntoView {
    view! {
      <body class="flex bg-lexodus-100 text-lexodus-900">
          // Global Vertical Icon Bar
          <div class="fixed flex h-screen w-16 flex-col items-center space-y-6 bg-lexodus-900 p-3 py-4 text-lexodus-200 hover:bg-lexodus-950">
              <a href="/" class="icon-bar-item relative flex flex-col items-center">
                  <span class="material-icons text-2xl">home</span>
                  <span class="tooltip">Home</span>
              </a>
              <a href="/case" class="icon-bar-item relative flex flex-col items-center">
                  <span class="material-icons text-2xl">folder</span>
                  <span class="tooltip">Cases</span>
              </a>
              <a href="/calendar" class="icon-bar-item relative flex flex-col items-center">
                  <span class="material-icons text-2xl">calendar_today</span>
                  <span class="tooltip">Calendar</span>
              </a>
              <a href="/documents" class="icon-bar-item relative flex flex-col items-center">
                  <span class="material-icons text-2xl">description</span>
                  <span class="tooltip">Documents</span>
              </a>
              <a href="/users" class="icon-bar-item relative flex flex-col items-center">
                <span class="material-icons text-2xl">settings</span>
                  <span class="tooltip"></span>
              </a>
          </div>

          // Page Layout with Live Feed as a Right Sidebar
          <div class="flex flex-1">

              // Main Content Wrapper (Shifted Right by Icon Bar Width)
              <div class="ml-16 flex flex-1 flex-col">

                  // Header Section for Notifications
                  <header class="flex items-center justify-between bg-white p-4 shadow-md">
                      <h1 class="text-2xl font-bold text-lexodus-800">Lexodus</h1>
                      <div class="relative">
                          <button aria-label="Notifications" class="focus:outline-none">
                              <span class="material-icons text-2xl text-lexodus-600">notifications</span>
                              <span class="absolute right-0 top-0 rounded-full bg-red-600 px-2 py-1 text-xs font-bold text-white">3</span>
                          </button>
                      </div>
                  </header>

                  // Main Content Area
                  <main id="main-content" class="overflow-auto bg-lexodus-100 p-6">
                      <nav aria-label="Breadcrumb" class="mb-6 text-sm text-lexodus-600">
                          <a href="#" class="hover:underline focus:underline">Home</a> /
                          <span class="text-lexodus-400">Cases</span>
                      </nav>


                      <section class="mb-6 rounded-lg border border-lexodus-200 bg-white p-6 shadow-lg">
                        {children()}
                        </section>
                  </main>
              </div>

              // Live Feed Sidebar on the Right
              <aside class="w-64 bg-white border-l border-lexodus-200 p-4 overflow-y-auto">
                <h2 class="text-xl font-semibold mb-4 text-lexodus-800">Recent Activity</h2>
                <ul class="space-y-4">

                  // Chat message with refined action toggles
                  <li class="bg-lexodus-50 p-3 rounded shadow">
                    <div class="flex items-center mb-2">
                      <img src="https://i.pravatar.cc/40?img=1" alt="John Snowe" class="w-8 h-8 rounded-full mr-2"/>
                      <p class="font-medium text-lexodus-700">John Snowe</p>
                    </div>
                    <p class="text-sm text-lexodus-600 mb-2">"I've filed the motion for Case #54321"</p>
                    <div class="flex space-x-2">

                      // Add Note Toggle
                      <div class="relative">
                        <input type="checkbox" id="show-notes-toggle" class="hidden peer"/>
                        <label for="show-notes-toggle" class="text-xs bg-lexodus-200 hover:bg-lexodus-300 text-lexodus-700 py-1 px-2 rounded cursor-pointer transition duration-150 ease-in-out">
                          Add Note
                        </label>

                        // Notes Content
                        <div class="hidden peer-checked:block absolute left-0 mt-2 w-64 bg-white border border-lexodus-200 rounded shadow-lg z-10">
                          <div class="p-2">
                            <h4 class="font-medium text-lexodus-700 mb-2">Notes:</h4>
                            <ul class="space-y-2 max-h-40 overflow-y-auto">
                              <li class="text-xs text-lexodus-600 bg-lexodus-50 p-2 rounded">
                                <p class="font-medium">Jane Doe (2 hours ago):</p>
                                <p>Reviewed the motion. Looks good to proceed.</p>
                              </li>
                              <li class="text-xs text-lexodus-600 bg-lexodus-50 p-2 rounded">
                                <p class="font-medium">Mike Johnson (Yesterday):</p>
                                <p>Discussed strategy with the client. They agree with our approach.</p>
                              </li>
                              <li class="text-xs text-lexodus-600 bg-lexodus-50 p-2 rounded">
                                <p class="font-medium">Sarah Smith (2 days ago):</p>
                                <p>Started drafting the motion. Will need review by senior partner.</p>
                              </li>
                            </ul>
                            <div class="mt-2">
                              <input type="text" placeholder="Add a new note..." class="w-full text-xs border border-lexodus-300 rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-lexodus-500"/>
                            </div>
                          </div>
                        </div>
                      </div>

                      // Schedule Hearing Toggle
                      <div class="relative">
                        <input type="checkbox" id="show-schedule-toggle" class="hidden peer"/>
                        <label for="show-schedule-toggle" class="text-xs bg-lexodus-200 hover:bg-lexodus-300 text-lexodus-700 py-1 px-2 rounded cursor-pointer transition duration-150 ease-in-out">
                          Schedule Hearing
                        </label>

                        // Schedule Hearing Content
                        <div class="hidden peer-checked:block absolute left-0 mt-2 w-64 bg-white border border-lexodus-200 rounded shadow-lg z-10">
                          <div class="p-2">
                            <h4 class="font-medium text-lexodus-700 mb-2">Schedule Hearing:</h4>
                            <form class="space-y-2">
                              <div>
                                <label for="hearing-date" class="block text-xs text-lexodus-600">Date:</label>
                                <input type="date" id="hearing-date" class="w-full text-xs border border-lexodus-300 rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-lexodus-500"/>
                              </div>
                              <div>
                                <label for="hearing-time" class="block text-xs text-lexodus-600">Time:</label>
                                <input type="time" id="hearing-time" class="w-full text-xs border border-lexodus-300 rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-lexodus-500"/>
                              </div>
                              <div>
                                <label for="hearing-type" class="block text-xs text-lexodus-600">Type:</label>
                                <select id="hearing-type" class="w-full text-xs border border-lexodus-300 rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-lexodus-500">
                                  <option>Motion Hearing</option>
                                  <option>Status Conference</option>
                                  <option>Pre-trial Conference</option>
                                  <option>Trial</option>
                                </select>
                              </div>
                              <button type="submit" class="w-full bg-lexodus-500 hover:bg-lexodus-600 text-white text-xs py-1 px-2 rounded transition duration-150 ease-in-out">
                                Confirm Scheduling
                              </button>
                            </form>
                          </div>
                        </div>
                      </div>

                      // View Case Link
                      <a href="#case-54321" class="text-xs bg-lexodus-200 hover:bg-lexodus-300 text-lexodus-700 py-1 px-2 rounded transition duration-150 ease-in-out">View Case</a>
                    </div>
                    <span class="block text-lexodus-400 text-xs mt-2">5 minutes ago</span>
                  </li>

                  // Additional items such as Document Upload Notification, Case Status Update, and Calendar Event can follow the same structure

                </ul>
              </aside>

          </div>
      </body>

    }
}
