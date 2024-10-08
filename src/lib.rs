use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;
mod composables;

use crate::components::navbar::NavBar;
// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::portfolio::Portfolio;
use crate::pages::projects::allianz::Allianz;
use crate::pages::projects::allianz2::Allianz2;
use crate::pages::projects::{Projects, sumo_robot::SumoRobot, trimble::Trimble, fff::FFF};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Router>
            <div class="bg-white dark:bg-slate-900">
                <NavBar/>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/Portfolio" view=Portfolio/>
                    <Route path="/Projects" view=Projects/>
                    <Route path="/Projects/SumoRobot" view=SumoRobot/>
                    <Route path="/Projects/Trimble" view=Trimble/>
                    <Route path="/Projects/FFF" view=FFF/>
                    <Route path="/Projects/Allianz" view=Allianz/>
                    <Route path="/Projects/Allianz2" view=Allianz2/>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </div>
        </Router>
    }
}
