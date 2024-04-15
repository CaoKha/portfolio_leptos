use leptos::*;
use leptos_use::{
    use_color_mode, use_cycle_list_with_options, ColorMode, UseColorModeReturn,
    UseCycleListOptions, UseCycleListReturn,
};

#[component]
pub fn ThemeButton() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
    let UseCycleListReturn { state, next, .. } = use_cycle_list_with_options(
        vec![ColorMode::Light, ColorMode::Dark],
        UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
    );
    let toggleTheme = move |_| next();
    let svg = move || match state.get() {
        ColorMode::Light => view! {
            <svg
                id="theme-toggle-dark-icon"
                class="h-4 w-4"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                viewBox="0 0 18 20"
            >
                <path d="M17.8 13.75a1 1 0 0 0-.859-.5A7.488 7.488 0 0 1 10.52 2a1 1 0 0 0 0-.969A1.035 1.035 0 0 0 9.687.5h-.113a9.5 9.5 0 1 0 8.222 14.247 1 1 0 0 0 .004-.997Z"></path>
            </svg>
        },
        ColorMode::Dark => view! {
            <svg
                id="theme-toggle-light-icon"
                class="h-4 w-4"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                viewBox="0 0 20 20"
            >
                <path d="M10 15a5 5 0 1 0 0-10 5 5 0 0 0 0 10Zm0-11a1 1 0 0 0 1-1V1a1 1 0 0 0-2 0v2a1 1 0 0 0 1 1Zm0 12a1 1 0 0 0-1 1v2a1 1 0 1 0 2 0v-2a1 1 0 0 0-1-1ZM4.343 5.757a1 1 0 0 0 1.414-1.414L4.343 2.929a1 1 0 0 0-1.414 1.414l1.414 1.414Zm11.314 8.486a1 1 0 0 0-1.414 1.414l1.414 1.414a1 1 0 0 0 1.414-1.414l-1.414-1.414ZM4 10a1 1 0 0 0-1-1H1a1 1 0 0 0 0 2h2a1 1 0 0 0 1-1Zm15-1h-2a1 1 0 1 0 0 2h2a1 1 0 0 0 0-2ZM4.343 14.243l-1.414 1.414a1 1 0 1 0 1.414 1.414l1.414-1.414a1 1 0 0 0-1.414-1.414ZM14.95 6.05a1 1 0 0 0 .707-.293l1.414-1.414a1 1 0 1 0-1.414-1.414l-1.414 1.414a1 1 0 0 0 .707 1.707Z"></path>
            </svg>
        },
        // TODO: other themes for now is default to Light mode
        _ => view! {
            <svg
                id="theme-toggle-dark-icon"
                class="h-4 w-4"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                viewBox="0 0 18 20"
            >
                <path d="M17.8 13.75a1 1 0 0 0-.859-.5A7.488 7.488 0 0 1 10.52 2a1 1 0 0 0 0-.969A1.035 1.035 0 0 0 9.687.5h-.113a9.5 9.5 0 1 0 8.222 14.247 1 1 0 0 0 .004-.997Z"></path>
            </svg>
        },
    };

    view! {
        <button
            on:click=toggleTheme
            id="theme-toggle"
            data-tooltip-target="tooltip-toggle"
            type="button"
            class="inline-flex h-10 w-10 items-center justify-center rounded-lg p-2.5 text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-[#3B82F6]/[.15] dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-700"
        >
            {svg}
            <span class="sr-only">Toggle dark mode</span>
        </button>
    }
}
