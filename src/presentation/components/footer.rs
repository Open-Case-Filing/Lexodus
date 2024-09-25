use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white py-6 mt-auto w-full border-t-4 border-cyan-500">
        <div class="container mx-auto flex flex-col md:flex-row justify-between items-center">
            <div>
                <h2 class="text-2xl font-semibold">Open Case Filing System</h2>
                <p class="text-gray-400">Enhancing judicial efficiency through technology</p>
            </div>
            <div class="flex space-x-4">
                <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Facebook</a>
                <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Twitter</a>
                <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Linkedin</a>
                <a href="#" class="text-gray-400 hover:text-white transition-colors duration-300">Github</a>
            </div>
        </div>
        <div class="text-center text-gray-500 mt-4">
            &copy; 2024 Open Case Filing System. All rights reserved.
        </div>
    </footer>
    }
}