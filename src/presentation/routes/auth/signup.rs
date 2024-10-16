use crate::functions::auth::Signup;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Meta property="og:title" content="Sign Up"/>
        <Title text="Sign Up"/>
        <Meta name="description" content="Sign up for the site"/>
        <Meta property="og:description" content="Sign up for the site"/>
        <Meta
            property="og:image"
            content="/images/lexodus.jpg"
        />
        <div class="bg-gradient-to-r from-lexodus-800 to-blue-900 flex items-center justify-center min-h-screen flex-col">
            <div class="text-center mb-8">
                <h1 class="text-4xl font-extrabold text-white mb-2">"Lexodus"</h1>
                <p class="text-lg text-gray-300">"Enhancing judicial efficiency through technology"</p>
            </div>
            <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg shadow-lg w-full max-w-sm">
                <h2 class="text-2xl font-bold mb-6 text-center text-white">"Sign Up"</h2>
                <ActionForm action=action class="space-y-6">
                    <div class="mb-4">
                        <label
                            for="username"
                            class="block text-white text-sm font-bold mb-2"
                        >
                            "Username"
                        </label>
                        <div class="mt-1">
                            <input
                                id="username"
                                required
                                name="username"
                                type="text"
                                autoComplete="username"
                                aria-describedby="username-error"
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            />
                        </div>
                    </div>
                    <div class="mb-4">
                        <label
                            for="display_name"
                            class="block text-white text-sm font-bold mb-2"
                        >
                            "Displayed Name"
                        </label>
                        <div class="mt-1">
                            <input
                                id="display_name"
                                required
                                name="display_name"
                                type="text"
                                autoComplete="display_name"
                                aria-describedby="display_name-error"
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            />
                        </div>
                    </div>
                    <div class="mb-4">
                        <label
                            for="password"
                            class="block text-white text-sm font-bold mb-2"
                        >
                            "Password"
                        </label>
                        <div class="mt-1">
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autoComplete="new-password"
                                aria-describedby="password-error"
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            />
                        </div>
                    </div>
                    <div class="mb-4">
                        <label
                            for="password_confirmation"
                            class="block text-white text-sm font-bold mb-2"
                        >
                            "Confirm Password"
                        </label>
                        <div class="mt-1">
                            <input
                                id="password_confirmation"
                                name="password_confirmation"
                                type="password"
                                autoComplete="password_confirmation"
                                aria-describedby="password_confirmation_error"
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            />
                        </div>
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        "Sign Up"
                    </button>
                    <div class="flex items-center justify-between">
                        <div class="text-center text-sm text-gray-300">
                            "Already have an account?"
                        </div>
                        <a rel="external" class="text-blue-300 hover:text-blue-100 underline" href="/login">
                            "Log in"
                        </a>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}
