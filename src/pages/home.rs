use crate::{components::navbar::NavBar, composables::get_ua};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // let (count, set_count) = create_signal(0);
    let ua_info = get_ua().expect("user agent should not be empty");
    logging::log!("ua_info: {:?}", ua_info);
    let platform = ua_info.category;
    let bevy_iframe = move || view! { <p>{format!("You are on {}", platform)}</p> };

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
            <NavBar/>
            <main class="relative flex min-h-screen flex-col items-center justify-start p-4">
                {bevy_iframe}

            </main>
        </ErrorBoundary>
    }
}
