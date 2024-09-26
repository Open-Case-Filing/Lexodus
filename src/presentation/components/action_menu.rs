use leptos::*;

#[component]
pub fn Action_Menu() -> impl IntoView {
    view! {
        <div class="w-64 h-full bg-gray-800 p-4 overflow-y-auto flex-shrink-0 border-l border-r border-gray-700">
            <div class="space-y-4">
                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Upcoming Hearings"</h2>
                    <ul class="mt-4 text-gray-400">
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "Case X starts in 2 hours. "
                            <span class="text-cyan-500 cursor-pointer">"Calendar"</span>
                            " "
                            <span class="text-cyan-500 cursor-pointer">"Join Channel"</span>
                        </li>
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "Case Y starts in 4 hours. "
                            <span class="text-cyan-500 cursor-pointer">"View Indictment"</span>
                            " "
                            <span class="text-cyan-500 cursor-pointer">"Chat"</span>
                        </li>
                    </ul>
                </div>
                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Pending Motions"</h2>
                    <ul class="mt-4 text-gray-400">
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "Motion for Summary Judgement "
                            <span class="text-cyan-500 cursor-pointer">"Case X"</span>
                            " "
                            <span class="text-cyan-500 cursor-pointer">"Respond"</span>
                        </li>
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "Motion for Jury Trial "
                            <span class="text-cyan-500 cursor-pointer">"Case Y"</span>
                            " "
                            <span class="text-cyan-500 cursor-pointer">"Respond"</span>
                            " "
                            <span class="text-cyan-500 cursor-pointer">"Chat"</span>
                        </li>
                    </ul>
                </div>
                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Chat"</h2>
                    <ul class="mt-4 text-gray-400">
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "#United States v Harpool "
                            <span>"No indictment has been filed."</span>
                            " "
                            <span class="text-cyan-500 cursor-pointer">"Respond"</span>
                        </li>
                    </ul>
                </div>
                <div>
                    <h2 class="text-sm font-semibold text-cyan-500 uppercase tracking-wider">"Courtroom Assignments"</h2>
                    <ul class="mt-4 text-gray-400">
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "Case X - 11:00 AM - Courtroom 12"
                        </li>
                        <li class="mb-2 hover:text-white transition-colors duration-300">
                            "Case Y - 1:00 PM - Courtroom 7"
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
