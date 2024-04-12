use leptos::*;
use leptos_router::*;

use crate::components::theme_btn::ThemeButton;

#[component]
pub fn NavBar() -> impl IntoView {
    let nav_list = move || {
        ["Portfolio", "Projects", "Blogs"].map(|item| {
        view! {
            <A
                href=item
                class="mr-4 mt-0 inline-block text-right text-slate-500 no-underline hover:text-slate-900 dark:text-slate-200 dark:hover:text-white"
            >
                {item}
            </A>
        }}).collect_view()
    };

    view! {
        <div class="border-b-2 border-b-slate-100 bg-blue-500/10 font-sans antialiased dark:bg-gray-800/90">
            <nav class="flex flex-wrap items-center justify-between p-2">
                <div class="flex-no-shrink ml-4 mr-6 mt-1 flex items-center text-white">
                    <A href="">
                        <img
                            src="/images/avatar.jpeg"
                            alt="ava"
                            width="40"
                            height="40"
                            class="aspect-auto w-10 rounded-full ring-1 ring-gray-900/5"
                        />
                    </A>
                    <span class="text-xl font-semibold tracking-tight"></span>
                </div>
                <div class="flex w-auto flex-grow items-center justify-between">
                    <div class="flex-grow text-lg font-bold lg:text-xl">{nav_list}</div>
                    <div class="pr-1">
                        <ThemeButton/>
                    </div>
                </div>
            </nav>
        </div>
    }
}
