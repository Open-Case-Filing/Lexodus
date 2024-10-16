use leptos::*;

#[component]
pub fn DefaultLayout(children: Children) -> impl IntoView {
    view! {
      <div class="flex bg-lexodus-100 text-lexodus-900">
          // Global Vertical Icon Bar
          <div class="fixed flex h-screen w-16 flex-col items-center space-y-6 bg-lexodus-900 p-3 py-4 text-lexodus-200 hover:bg-lexodus-950">
              <a href="/" class="icon-bar-item relative flex flex-col items-center">
                  <span class="material-icons text-2xl">home</span>
                  <span class="tooltip">Home</span>
              </a>
              <a href="/cases" class="icon-bar-item relative flex flex-col items-center">
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



                      <section class="mb-6 rounded-lg border border-lexodus-200 bg-white p-6 shadow-lg">
                        {children()}
                        </section>
                  </main>
              </div>

              //  <!-- Live Feed Sidebar on the Right -->
               <aside class="w-64 overflow-y-auto border-l border-lexodus-200 bg-white p-4">
                 <h2 class="mb-4 text-xl font-semibold text-lexodus-800">Recent Activity</h2>
                 <ul class="space-y-4">
                  //  <!-- Chat message with refined action toggles -->
                   <li class="rounded bg-lexodus-50 p-3 shadow">
                     <div class="mb-2 flex items-center">
                       <img src="https://i.pravatar.cc/40?img=1" alt="John Snowe" class="mr-2 h-8 w-8 rounded-full" />
                       <p class="font-medium text-lexodus-700">John Snowe</p>
                     </div>
                     <p class="mb-2 text-sm text-lexodus-600">"I've filed the motion for Case #54321"</p>
                     <div class="flex space-x-2">
                       <div class="relative">
                         <input type="checkbox" id="show-notes-toggle" class="peer hidden" />
                         <label for="show-notes-toggle" class="inline-block cursor-pointer rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300"> Add Note </label>
                         <div class="absolute left-0 z-10 mt-2 hidden w-64 rounded border border-lexodus-200 bg-white shadow-lg peer-checked:block">
                           <div class="p-2">
                             <h4 class="mb-2 font-medium text-lexodus-700">Notes:</h4>
                             <ul class="max-h-40 space-y-2 overflow-y-auto">
                               <li class="rounded bg-lexodus-50 p-2 text-xs text-lexodus-600">
                                 <p class="font-medium">Jane Doe (2 hours ago):</p>
                                 <p>Reviewed the motion. Looks good to proceed.</p>
                               </li>
                               <li class="rounded bg-lexodus-50 p-2 text-xs text-lexodus-600">
                                 <p class="font-medium">Mike Johnson (Yesterday):</p>
                                 <p>Discussed strategy with the client. They agree with our approach.</p>
                               </li>
                               <li class="rounded bg-lexodus-50 p-2 text-xs text-lexodus-600">
                                 <p class="font-medium">Sarah Smith (2 days ago):</p>
                                 <p>Started drafting the motion. Will need review by senior partner.</p>
                               </li>
                             </ul>
                             <div class="mt-2">
                               <input type="text" placeholder="Add a new note..." class="w-full rounded border border-lexodus-300 px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-lexodus-500" />
                             </div>
                           </div>
                         </div>
                       </div>
                       <div class="relative">
                         <input type="checkbox" id="show-schedule-toggle" class="peer hidden" />
                         <label for="show-schedule-toggle" class="inline-block cursor-pointer rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300"> Schedule Hearing </label>
                         <div class="absolute left-0 z-10 mt-2 hidden w-64 rounded border border-lexodus-200 bg-white shadow-lg peer-checked:block">
                           <div class="p-2">
                             <h4 class="mb-2 font-medium text-lexodus-700">Schedule Hearing:</h4>
                             <form class="space-y-2">
                               <div>
                                 <label for="hearing-date" class="block text-xs text-lexodus-600">Date:</label>
                                 <input type="date" id="hearing-date" class="w-full rounded border border-lexodus-300 px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-lexodus-500" />
                               </div>
                               <div>
                                 <label for="hearing-time" class="block text-xs text-lexodus-600">Time:</label>
                                 <input type="time" id="hearing-time" class="w-full rounded border border-lexodus-300 px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-lexodus-500" />
                               </div>
                               <div>
                                 <label for="hearing-type" class="block text-xs text-lexodus-600">Type:</label>
                                 <select id="hearing-type" class="w-full rounded border border-lexodus-300 px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-lexodus-500">
                                   <option>Motion Hearing</option>
                                   <option>Status Conference</option>
                                   <option>Pre-trial Conference</option>
                                   <option>Trial</option>
                                 </select>
                               </div>
                               <button type="submit" class="w-full rounded bg-lexodus-500 px-2 py-1 text-xs text-white transition duration-150 ease-in-out hover:bg-lexodus-600">Confirm Scheduling</button>
                             </form>
                           </div>
                         </div>
                       </div>
                       <a href="#case-54321" class="rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300">View Case</a>
                     </div>
                     <span class="mt-2 block text-xs text-lexodus-400">5 minutes ago</span>
                   </li>

                  //  <!-- Document upload notification -->
                   <li class="rounded bg-lexodus-50 p-3 shadow">
                     <div class="mb-2 flex items-center">
                       <svg class="mr-2 h-6 w-6 text-lexodus-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                       <p class="font-medium text-lexodus-700">New Document</p>
                     </div>
                     <p class="mb-2 text-sm text-lexodus-600">Motion for Summary Judgment uploaded to Case #12345</p>
                     <div class="flex space-x-2">
                       <a href="#view-doc" class="rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300">View Document</a>
                       <button class="rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300">Download</button>
                     </div>
                     <span class="mt-2 block text-xs text-lexodus-400">20 minutes ago</span>
                   </li>

                  //  <!-- Case status update with toggle -->
                   <li class="rounded bg-lexodus-50 p-3 shadow">
                     <p class="mb-2 font-medium text-lexodus-700">Case #67890 Status Update</p>
                     <p class="mb-2 text-sm text-lexodus-600">Status changed from "Open" to "Closed"</p>
                     <div class="flex items-center">
                       <span class="mr-2 text-sm text-lexodus-600">Show reason:</span>
                       <input type="checkbox" id="show-reason-toggle" class="hidden" />
                       <label for="show-reason-toggle" class="flex cursor-pointer items-center">
                         <div class="relative">
                           <div class="h-4 w-10 rounded-full bg-lexodus-400 shadow-inner"></div>
                           <div class="dot absolute -left-1 -top-1 h-6 w-6 rounded-full bg-white shadow transition"></div>
                         </div>
                       </label>
                     </div>
                     <p class="reason-content mt-2 hidden text-sm text-lexodus-600">Case resolved: Settlement agreement reached between parties.</p>
                     <span class="mt-2 block text-xs text-lexodus-400">1 hour ago</span>
                   </li>

                  //  <!-- Calendar event reminder with court calendar link -->
                   <li class="rounded bg-lexodus-50 p-3 shadow">
                     <div class="mb-2 flex items-center">
                       <svg class="mr-2 h-6 w-6 text-lexodus-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                       <p class="font-medium text-lexodus-700">Upcoming Hearing</p>
                     </div>
                     <p class="mb-2 text-sm text-lexodus-600">Preliminary hearing for Case #98765 scheduled tomorrow at 10:00 AM</p>
                     <div class="flex space-x-2">
                       <button class="rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300">Set Reminder</button>
                       <a href="#case-98765" class="rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300">View Case</a>
                       <a href="#court-calendar" class="rounded bg-lexodus-200 px-2 py-1 text-xs text-lexodus-700 transition duration-150 ease-in-out hover:bg-lexodus-300">Court Calendar</a>
                     </div>
                     <span class="mt-2 block text-xs text-lexodus-400">2 hours ago</span>
                   </li>
                 </ul>
               </aside>

          </div>
      </div>

    }
}
