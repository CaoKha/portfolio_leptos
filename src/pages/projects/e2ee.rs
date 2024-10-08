use leptos::*;

#[component]
pub fn E2ee() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        "Rust CLI for E2E encryption"
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200">Done in 2 days</p>
                    <br/>
                    <img src="/images/lock.jpeg" class="m-auto lg:w-1/3"/>
                    <p class="text-center dark:text-slate-400 sm:text-xl pt-4 lg:pt-10">
                        "This project involves writing an simple E2E encryption CLI in Rust using rsa crate.
                        The goal is to make it and ship it as soon as possible. The CLI supports Linux, MacOS, and Windows."
                    </p>
                    <br/>
                    <div class="text-center">
                        <button class="rounded-full bg-slate-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
                            <a href="https://github.com/CaoKha/e2e_encryption" target="_blank">
                                <span>Demo Code</span>
                            </a>
                        </button>
                    </div>
                </div>
            </section>
        </main>
    }
}
