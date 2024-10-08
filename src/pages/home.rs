// use crate::composables::get_ua;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let yogurt_iframe = move || {
        view! {
            <span class="text-base text-slate-700 dark:text-slate-200">
                <iframe class="w-[480px] h-[360px]" id="yogurt-game" src="/yogurt/index.html" allow="fullscreen"></iframe>
                </span>
        }
    };
    let marble_iframe = move || {
        view! {
            <span class="text-base text-slate-700 dark:text-slate-200">
                <iframe class="w-[480px] h-[360px]" id="marble-game" src="/marble/index.html" allow="fullscreen"></iframe>
                </span>
        }
    };
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong in Home!"</h1>
                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <main class="relative flex min-h-screen flex-col items-center justify-start p-4">
                <h2 class="text-lg font-bold tracking-tight dark:text-slate-200 sm:text-xl">Yogurt Placeholder</h2>
                <div class="mb-2">
                {yogurt_iframe}
                </div>
                <h2 class="text-lg font-bold tracking-tight dark:text-slate-200 sm:text-xl">Marble</h2>
                <div>
                {marble_iframe}
                </div>
            </main>
        </ErrorBoundary>
    }
}
