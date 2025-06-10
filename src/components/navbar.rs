use leptos::*;
use leptos_router::*;

use crate::components::theme_btn::ThemeButton;
use crate::composables::NAV_ITEMS;

#[component]
pub fn NavBar() -> impl IntoView {
    let nav_list = move || {
        NAV_ITEMS.map(|item| {
        view! {
            <A
                href=format!("/{}", item)
                class="mr-4 mt-0 inline-block text-right text-slate-500 no-underline hover:text-slate-900 dark:text-slate-200 dark:hover:text-white"
            >
                {item}
            </A>
        }}).collect_view()
    };

    view! {
        <div class="border-b-2 border-b-slate-100 bg-blue-500/10 font-sans antialiased dark:bg-gray-800/90">
            <nav class="flex flex-wrap items-center justify-between p-2">
                <div class="flex-no-shrink ml-0 sm:ml-4 mr-4 sm:mr-6 mt-1 flex items-center text-white">
                    <A href="">
                        <img
                            src="/images/avatar.jpeg"
                            alt="ava"
                            class="aspect-auto w-10 rounded-full ring-1 ring-gray-900/5"
                        />
                    </A>
                    <span class="text-xl font-semibold tracking-tight"></span>
                </div>
                <div class="flex w-auto flex-grow items-center justify-between">
                    <div class="flex-grow text-lg font-bold lg:text-xl">{nav_list}</div>
                    <div class="pr-1 flex">
                        <ul class="ml-1 flex items-center" aria-label="Github">
                            <li class="mr-1 text-xs">
                                <button
                                    id="Github"
                                    type="button"
                                    class="inline-flex h-10 w-10 items-center justify-center rounded-lg p-2.5 text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-[#3B82F6]/[.15] dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-700"
                                >

                                    <a
                                        class="block hover:text-slate-700 dark:text-slate-200 dark:hover:text-slate-400"
                                        href="https://github.com/CaoKha"
                                        target="_blank"
                                        rel="noreferrer"
                                    >
                                        <span class="sr-only">GitHub</span>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 16 16"
                                            fill="currentColor"
                                            class="h-6 w-6"
                                            aria-hidden="true"
                                        >
                                            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                                        </svg>
                                    </a>
                                </button>
                            </li>
                            <li class="mr-1 text-xs">

                                <button
                                    id="LinkedIn"
                                    type="button"
                                    class="inline-flex h-10 w-10 items-center justify-center rounded-lg p-2.5 text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-[#3B82F6]/[.15] dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-700"
                                >
                                    <a
                                        class="block hover:text-slate-700 dark:text-slate-200 dark:hover:text-slate-400"
                                        href="https://www.linkedin.com/in/cao-kha-n-11327a239/"
                                        target="_blank"
                                        rel="noreferrer"
                                    >
                                        <span class="sr-only">LinkedIn</span>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                            class="h-6 w-6"
                                            aria-hidden="true"
                                        >
                                            <path d="M20.5 2h-17A1.5 1.5 0 002 3.5v17A1.5 1.5 0 003.5 22h17a1.5 1.5 0 001.5-1.5v-17A1.5 1.5 0 0020.5 2zM8 19H5v-9h3zM6.5 8.25A1.75 1.75 0 118.3 6.5a1.78 1.78 0 01-1.8 1.75zM19 19h-3v-4.74c0-1.42-.6-1.93-1.38-1.93A1.74 1.74 0 0013 14.19a.66.66 0 000 .14V19h-3v-9h2.9v1.3a3.11 3.11 0 012.7-1.4c1.55 0 3.36.86 3.36 3.66z"></path>
                                        </svg>
                                    </a>
                                </button>
                            </li>
                        </ul>

                        <ThemeButton/>
                    </div>
                </div>
            </nav>
        </div>
    }
}
