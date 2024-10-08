use leptos::*;
use crate::components::project_card::ProjectCard;
use crate::composables::PROJECTS;
pub mod sumo_robot;
pub mod trimble;
pub mod fff;
pub mod allianz;
pub mod etl;
pub mod e2ee;

#[component]
pub fn Projects() -> impl IntoView {
    let projects_list = move || {
        PROJECTS.map(|project| {
            view! {
                <li class="mb-12 px-2">
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
        <main class="min-h-screen py-10">
            <section
                id="Projects"
                class="mb-16 scroll-mt-16 md:mb-24 lg:mb-36 lg:scroll-mt-24"
                aria-label="Selected projects"
            >
                <div>
                    <ul>{projects_list}</ul>
                </div>
            </section>
        </main>
    }
}
