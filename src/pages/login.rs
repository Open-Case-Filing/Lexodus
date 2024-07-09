use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {

        
        <div class="bg-gradient-to-r from-cyan-800 to-blue-900 flex items-center justify-center min-h-screen flex-col">
        <div class="text-center mb-8">
            <h1 class="text-4xl font-extrabold text-white mb-2">"Open Case Filing System"</h1>
            <p class="text-lg text-gray-300">"Enhancing judicial efficiency through technology"</p>
        </div>
        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg shadow-lg w-full max-w-sm">
            <h2 class="text-2xl font-bold mb-6 text-center text-white">"Login"</h2>
            <form action="/dashboard/overview">
                <div class="mb-4">
                    <label for="username" class="block text-white text-sm font-bold mb-2">"Username"</label>
                    <input type="text" id="username" name="username" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" required/>
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-white text-sm font-bold mb-2">"Password"</label>
                    <input type="password" id="password" name="password" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" required/>
                </div>

                <div class="flex items-center justify-between">
                    <button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                        "Login"
                    </button>
                </div>
            </form>
        </div>
    </div>
    }
}