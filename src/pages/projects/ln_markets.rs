use leptos::*;

const STACK: [&str; 11] = [
    "TypeScript",
    "Rust",
    "Python",
    "React",
    "oRPC",
    "Tailwind CSS",
    "Vite",
    "Vitest",
    "Playwright",
    "GitHub Actions",
    "Docker",
];

#[component]
pub fn LnMarkets() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        "LN Markets — Bitcoin derivatives platform"
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Ito, Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200">"Jan 2026 – Present"</p>

                    <p class="pt-4 text-center dark:text-slate-400 sm:text-xl lg:pt-10">
                        "Backend and full-stack features on the Bitcoin derivatives trading platform, from type-safe API contracts to the qAssets referral programme."
                    </p>

                    <ul class="mx-auto mt-6 flex max-w-3xl flex-wrap justify-center" aria-label="Technologies used">
                        {STACK
                            .map(|tech| {
                                view! {
                                    <li class="mr-1.5 mt-2">
                                        <div class="flex items-center rounded-full bg-teal-400/10 px-3 py-1 text-xs font-medium leading-5 text-teal-700 dark:text-teal-300">
                                            {tech}
                                        </div>
                                    </li>
                                }
                            })}
                    </ul>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-32">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "1: REST APIs and platform features"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Developing backend and full-stack features in TypeScript, Rust and Python. Built REST APIs, delivered the announcement banner flow, and implemented trusted-device and session management."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "2: qAssets referral programme"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Building the referral programme for qAssets: gold, stocks, an index and oil, margined and settled in sats."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "3: Type-safe APIs with oRPC"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Built the backend with oRPC: end-to-end type-safe API contracts shared by server and client, removing a separate API schema layer."
                        </p>
                    </div>
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "4: React frontend"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Built the frontend in React and TypeScript with Tailwind CSS."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "5: Testing and delivery"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Tested with Vitest for unit tests and Playwright for end-to-end tests, shipped with GitHub Actions and Docker."
                        </p>
                        <br/>
                        <button class="rounded-full bg-slate-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
                            <a href="https://lnmarkets.com/" target="_blank" rel="noreferrer">
                                <span>"LN Markets"</span>
                            </a>
                        </button>
                    </div>
                </div>
            </section>
        </main>
    }
}
