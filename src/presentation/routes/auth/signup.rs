use crate::functions::auth::Signup;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Meta property="og:title" content="Signup"/>
        <Title text="Signup"/>
        <Meta name="description" content="Signup for the site"/>
        <Meta property="og:description" content="Signup for the site"/>

        <div class="bg-gradient-to-r from-cyan-800 to-blue-900 flex items-center justify-center min-h-screen flex-col">
            <div class="text-center mb-8">
                <h1 class="text-4xl font-extrabold text-white mb-2">"Lexodus"</h1>
                <p class="text-lg text-gray-300">"Enhancing judicial efficiency through technology"</p>
            </div>
            <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg shadow-lg w-full max-w-sm">
                <h2 class="text-2xl font-bold mb-6 text-center text-white">"Sign Up"</h2>
                <ActionForm action=action class="space-y-6">
                    <div class="mb-4">
                        <label for="username" class="block text-white text-sm font-bold mb-2">"Username"</label>
                        <input type="text" id="username" name="username" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" required/>
                    </div>
                    <div class="mb-4">
                        <label for="display_name" class="block text-white text-sm font-bold mb-2">"Displayed Name"</label>
                        <input type="text" id="display_name" name="display_name" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" required/>
                    </div>
                    <div class="mb-4">
                        <label for="password" class="block text-white text-sm font-bold mb-2">"Password"</label>
                        <input type="password" id="password" name="password" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" required/>
                    </div>
                    <div class="mb-4">
                        <label for="password_confirmation" class="block text-white text-sm font-bold mb-2">"Confirm Password"</label>
                        <input type="password" id="password_confirmation" name="password_confirmation" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" required/>
                    </div>
                    <div class="flex items-center justify-between">
                        <button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline w-full">
                            "Create Account"
                        </button>
                    </div>
                    <div class="flex items-center justify-center">
                        <div class="text-center text-sm text-gray-300">
                            "Already have an account? "
                            <a rel="external" class="text-blue-300 underline" href="/login">
                                "Log in"
                            </a>
                        </div>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}