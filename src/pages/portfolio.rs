use leptos::html::Section;
use leptos::*;
use leptos_router::A;
use leptos_use::use_element_visibility;

use crate::components::{experience_card::ExperienceCard, project_card::ProjectCard};
use crate::composables::{EXPERIENCES, HEADERS, PROJECTS};

#[component]
pub fn Portfolio() -> impl IntoView {
    let target_about = create_node_ref::<Section>();
    let target_exp = create_node_ref::<Section>();
    let target_proj = create_node_ref::<Section>();
    let is_about_visible = use_element_visibility(target_about);
    let is_exp_visible = use_element_visibility(target_exp);
    let is_proj_visible = use_element_visibility(target_proj);
    let current_visible = move || match [is_about_visible(), is_exp_visible(), is_proj_visible()] {
        [true, false, false] => "About",
        [true, true, false] => "About",
        [false, true, false] => "Experiences",
        [false, true, true] => "Projects",
        [false, false, true] => "Projects",
        _ => "",
    };

    let header_list = move || {
        HEADERS.map(|header| {
        view! {
            <span class=("active", move || current_visible() == header)>
                <A class="group flex items-center py-3" href=format!("#{}", header)>
                    <span class="nav-indicator mr-4 h-px w-8 bg-slate-600 transition-all active:bg-slate-700 group-hover:w-16 group-hover:bg-slate-700 group-focus-visible:w-16 group-focus-visible:bg-slate-200 motion-reduce:transition-none dark:group-hover:bg-slate-200"></span>
                    <span class="nav-text text-xs font-bold uppercase tracking-widest text-slate-500 group-hover:text-slate-700 group-focus-visible:text-slate-200 dark:group-hover:text-slate-200">
                        {header}
                    </span>
                </A>
            </span>
        }
    })
    };

    let experience_list = move || {
        EXPERIENCES.map(|experience| {
            view! {
                <li class="mb-12">
                    <ExperienceCard
                        date=experience.date
                        job=experience.job
                        company=experience.company_name
                        link=experience.link
                        content=experience.content
                        tech_stack=experience.tech_stack
                    />
                </li>
            }
        })
    };

    let projects_list = move || {
        PROJECTS.map(|project| {
            view! {
                <li class="mb-12">
                    <ProjectCard
                        title=project.title
                        link=project.link
                        description=project.description
                        img_src=project.img_src
                        tech_stack=project.tech_stack
                    />
                </li>
            }
        })
    };

    view! {
        <div class="min-h-screen p-6">
            <div class="lg:mx-auto lg:flex lg:justify-between lg:gap-4">
                <header class="lg:sticky lg:top-0 lg:flex lg:max-h-screen lg:w-1/4 lg:flex-col lg:pt-10">
                    <div>
                        <h1 class="text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                            Kha Nguyen
                        </h1>
                        <h2 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                            Software Engineer
                        </h2>
                        <p class="mt-4 max-w-xs leading-normal text-slate-400"></p>
                        <nav class="nav hidden lg:block" aria-label="In-page jump links">
                            <ul class="mt-16 w-max">
                                <li>{header_list}</li>
                            </ul>
                        </nav>
                    </div>
                </header>
                <main class="pt-24 lg:w-3/4 lg:pt-10">

                    <section
                        node_ref=target_about
                        id="About"
                        class="mb-16 scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24"
                        aria-label="About me"
                    >
                        <div class="sticky top-0 z-20 -mx-6 mb-4 w-screen bg-blue-500/10 px-6 py-5 backdrop-blur dark:bg-slate-900/75 md:pr-12 lg:sr-only lg:relative lg:top-auto lg:mx-auto lg:w-full lg:px-0 lg:py-0 lg:opacity-0">
                            <h2 class="text-sm font-bold uppercase tracking-widest text-slate-700 dark:text-slate-200 lg:sr-only">
                                About
                            </h2>
                        </div>
                        <div class="text-base text-slate-700 dark:text-slate-200">
                            <p class="mb-2">
                                "Hi, I studied mechatronics at Univeristy of Technologies of Compiegne (UTC) in France and I'm working
                                on a graphical project."
                            </p>
                            <p class="mb-2">
                                "Try clicking on the dino on the top left corner if you would like to see my experiment! ;)"
                            </p>
                            <p>
                                "When I’m not at the computer, I’m usually hanging out with my friend, playing badminton/tennis, watching movies or
                                running around Paris searching for stuffs"
                            </p>
                        </div>
                    </section>

                    <section
                        id="Experiences"
                        class="mb-16 scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24"
                        aria-label="Work experience"
                        node_ref=target_exp
                    >
                        <div class="sticky top-0 z-20 -mx-6 mb-4 w-screen bg-blue-500/10 px-6 py-5 backdrop-blur dark:bg-slate-900/75 md:pr-12 lg:sr-only lg:relative lg:top-auto lg:mx-auto lg:w-full lg:px-0 lg:py-0 lg:opacity-0">
                            <h2 class="text-sm font-bold uppercase tracking-widest text-slate-700 dark:text-slate-200 lg:sr-only">
                                Experience
                            </h2>
                        </div>
                        <div>
                            <ol class="group/list">{experience_list}</ol>
                            <div class="mt-12">
                                <a
                                    class="group inline-flex items-center font-semibold leading-tight text-slate-500 dark:text-slate-200"
                                    aria-label="View Full CV"
                                    href="/pdf/resume.pdf"
                                    target="_blank"
                                >
                                    <span>
                                        <span class="border-b border-transparent pb-px transition group-hover:border-teal-300 motion-reduce:transition-none">
                                            "View
                                            Full"
                                        </span>
                                        <span class="whitespace-nowrap">
                                            <span class="border-b border-transparent pb-px transition group-hover:border-teal-300 motion-reduce:transition-none">
                                                CV
                                            </span>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                viewBox="0 0 20 20"
                                                fill="currentColor"
                                                class="ml-1 inline-block h-4 w-4 shrink-0 -translate-y-px transition-transform group-hover:translate-x-2 group-focus-visible:translate-x-2 motion-reduce:transition-none"
                                                aria-hidden="true"
                                            >
                                                <path
                                                    fill-rule="evenodd"
                                                    d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z"
                                                    clip-rule="evenodd"
                                                ></path>
                                            </svg>
                                        </span>
                                    </span>
                                </a>
                            </div>
                        </div>
                    </section>

                    <section
                        id="Projects"
                        class="mb-16 scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24"
                        aria-label="Selected projects"
                        node_ref=target_proj
                    >
                        <div class="sticky top-0 z-20 -mx-6 mb-4 w-screen bg-blue-500/10 px-6 py-5 backdrop-blur dark:bg-slate-900/75 md:pr-12 lg:sr-only lg:relative lg:top-auto lg:mx-auto lg:w-full lg:px-0 lg:py-0 lg:opacity-0">
                            <h2 class="text-sm font-bold uppercase tracking-widest text-slate-700 dark:text-slate-200 lg:sr-only">
                                Projects
                            </h2>
                        </div>
                        <div>
                            <ul class="group/list">{projects_list}</ul>
                        </div>
                    </section>
                </main>

            </div>
        </div>
    }
}
