use crate::composables::get_ua;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // let (count, set_count) = create_signal(0);
    // let ua_info = get_ua().expect("user agent should not be empty");
    // let platform = ua_info.category;
    // let bevy_iframe = move || if platform.to_uppercase() != "PC" { 
    //     view! {
    //     <span class="text-base text-slate-700 dark:text-slate-200">
    //         <p>
    //             {format!(
    //                 "You are currently on a \"{}\". Hence, you can't see the gizmo game. 
    //                 Try to access this page with a laptop to see the gizmo!",
    //                 platform.to_uppercase(),
    //             )}
    //
    //         </p>
    //         <p>"This website is made with Leptos, a Rust framework for web apps"</p>
    //         <br/>
    //         <span class="flex justify-center">
    //             <button class="rounded-full bg-slate-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
    //                 <a href="https://nckportfolio.vercel.app" target="_blank">
    //                     <span>Migrating from Nuxt 3</span>
    //                 </a>
    //             </button>
    //         </span>
    //
    //     </span>
    //     }
    // } else {
    //     view! {
    //     <span class="text-base text-slate-700 dark:text-slate-200">
    //         <iframe class="w-[1280px] h-[720px]" id="bevy-game" src="/game/index.html" allow="fullscreen"></iframe>
    //     </span>
    //     }
    //  };
    let bevy_iframe = move || {
        view! {
            <span class="text-base text-slate-700 dark:text-slate-200">
                <iframe class="w-[480px] h-[360px]" id="bevy-game" src="/game/index.html" allow="fullscreen"></iframe>
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
                {bevy_iframe}
            </main>
        </ErrorBoundary>
    }
}
