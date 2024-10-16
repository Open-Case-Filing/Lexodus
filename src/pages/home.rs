use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
        <div class="bg-gradient-to-r from-slate-800 to-slate-900 min-h-screen">
                <div class="container mx-auto px-4 py-16">
                    <header class="text-center mb-16">
                        <h1 class="text-5xl font-bold text-white mb-4">"Welcome to Lexodus"</h1>
                        <p class="text-xl text-gray-300">"Next-Generation Federal Court Case Management"</p>
                    </header>

                    <div class="grid md:grid-cols-2 gap-12 items-center mb-16">
                        <div class="text-white">
                            <h2 class="text-3xl font-semibold mb-6">"Modernizing Federal Court Operations"</h2>
                            <p class="mb-8">"Lexodus is an advanced case management and electronic court filing system designed to streamline federal court processes, enhance accessibility, and improve judicial efficiency."</p>
                            <div class="space-x-4">
                            <a href="/login" class="bg-slate-600 hover:bg-slate-700 text-white font-bold py-3 px-6 rounded-lg transition duration-300">"Login"</a>
                                <a href="/signup" class="bg-transparent hover:bg-white hover:text-slate-900 text-white font-bold py-3 px-6 rounded-lg border border-white transition duration-300">"Register"</a>
                            </div>
                        </div>
                        <div class="hidden md:block">
                            <img src="/images/lexodus-dashboard.png" alt="Lexodus Dashboard" class="rounded-lg shadow-2xl"/>
                        </div>
                    </div>

                    <div class="mt-24 grid md:grid-cols-3 gap-8 text-center">
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg">
                            <i class="fas fa-gavel text-4xl text-slate-300 mb-4"></i>
                            <h3 class="text-xl font-semibold text-white mb-2">"Electronic Filing"</h3>
                            <p class="text-gray-300">"Securely file and manage court documents electronically 24/7"</p>
                        </div>
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg">
                            <i class="fas fa-search text-4xl text-slate-300 mb-4"></i>
                            <h3 class="text-xl font-semibold text-white mb-2">"Case Search & Tracking"</h3>
                            <p class="text-gray-300">"Easily search, view, and track federal court cases and dockets"</p>
                        </div>
                        <div class="bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg">
                            <i class="fas fa-chart-line text-4xl text-slate-300 mb-4"></i>
                            <h3 class="text-xl font-semibold text-white mb-2">"Analytics & Reporting"</h3>
                            <p class="text-gray-300">"Generate insights with advanced analytics and custom reports"</p>
                        </div>
                    </div>

                    <div class="mt-24 grid md:grid-cols-2 gap-12 items-center">
                        <div class="text-white">
                            <h2 class="text-3xl font-semibold mb-6">"Empowering Federal Courts"</h2>
                            <p class="mb-8">"Lexodus provides a comprehensive suite of tools for judges, clerks, attorneys, and the public to interact seamlessly with the federal court system."</p>
                            <ul class="list-disc list-inside mb-8">
                                <li>"Automated case assignment and management"</li>
                                <li>"Integrated calendaring and scheduling"</li>
                                <li>"Secure document storage and retrieval"</li>
                                <li>"Role-based access control"</li>
                                <li>"Public access to court records (PACER-like functionality)"</li>
                            </ul>
                        </div>
                        <div class="hidden md:block">
                            <img src="/images/lexodus-features.png" alt="Lexodus Features" class="rounded-lg shadow-2xl"/>
                        </div>
                    </div>

                    <div class="mt-24 grid md:grid-cols-2 gap-12 items-center">
                        <div class="hidden md:block">
                            <img src="/images/lexodus-chat-collaboration.png" alt="Lexodus Chat and Collaboration" class="rounded-lg shadow-2xl"/>
                        </div>
                        <div class="text-white">
                            <h2 class="text-3xl font-semibold mb-6">"Enhanced Collaboration"</h2>
                            <p class="mb-8">"Lexodus revolutionizes court communications with integrated chat and collaboration tools, enabling secure and efficient interactions between judges, clerks, and authorized personnel."</p>
                            <ul class="list-disc list-inside mb-8">
                                <li>"Real-time messaging for quick consultations"</li>
                                <li>"Secure file sharing within the platform"</li>
                                <li>"Collaborative document editing for orders and opinions"</li>
                                <li>"Task assignment and tracking for court staff"</li>
                                <li>"Audit trails for all communications and actions"</li>
                            </ul>
                        </div>
                    </div>

                                        <div class="mt-24 grid md:grid-cols-2 gap-12 items-center">
                        <div class="text-white">
                            <h2 class="text-3xl font-semibold mb-6">"Unparalleled Security for Sealed Documents"</h2>
                            <p class="mb-8">"Lexodus sets a new standard in document security, employing state-of-the-art cryptographic algorithms to protect sealed and confidential court documents."</p>
                            <ul class="list-disc list-inside mb-8">
                                <li>"Advanced encryption using AES-256 in GCM mode"</li>
                                <li>"Elliptic Curve Cryptography (ECC) for key exchange"</li>
                                <li>"Multi-factor authentication for accessing sealed documents"</li>
                                <li>"Granular access controls with detailed audit logging"</li>
                                <li>"Secure key management with hardware security modules (HSMs)"</li>
                            </ul>
                            <p class="mb-4">"Our system employs post-quantum cryptographic algorithms, ensuring long-term security even against future quantum computer attacks."</p>
                        </div>
                        <div class="hidden md:block">
                            <img src="/images/lexodus-sealed-documents.png" alt="Lexodus Sealed Documents Security" class="rounded-lg shadow-2xl"/>
                        </div>
                    </div>

                    <div class="mt-12 bg-white bg-opacity-10 backdrop-filter backdrop-blur-lg p-8 rounded-lg text-white">
                        <h3 class="text-2xl font-semibold mb-4">"Commitment to Confidentiality"</h3>
                        <p>"Lexodus is designed with a 'security-first' approach, ensuring that sealed documents and sensitive court information remain confidential. Our system adheres to the highest standards of data protection, complying with federal regulations and court security requirements."</p>
                    </div>

                    <footer class="text-center mt-16">
                        <p class="text-gray-300">
                        "For assistance, please contact the "
                        <a rel="external" href="https://github.com/open-case-filing/lexodus">Lexodus Team</a>
                        </p>
                    </footer>
                </div>
            </div>
        </div>
    }
}
