use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod composables;
mod pages;

use crate::components::navbar::NavBar;
use crate::composables::base_path;
// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::portfolio::Portfolio;
use crate::pages::projects::allianz::Allianz;
use crate::pages::projects::dlc::DlcMarkets;
use crate::pages::projects::e2ee::E2ee;
use crate::pages::projects::etl::Etl;
use crate::pages::projects::ln_markets::LnMarkets;
use crate::pages::projects::{fff::FFF, sumo_robot::SumoRobot, trimble::Trimble, Projects};

/// An app router which renders the portfolio and handles 404's.
///
/// Routes carry the deployment base path themselves because GitHub Pages project
/// sites are served from `/<repo>/`, and the router's own base handling does not
/// strip the trailing slash that Pages uses for directory URLs.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let base = base_path();

    view! {
        <Router trailing_slash=TrailingSlash::Exact>
            <div class="bg-white dark:bg-slate-900">
                <NavBar/>
                <Routes>
                    <Route path=format!("{base}/") view=Portfolio/>
                    <Route path=format!("{base}/Portfolio") view=Portfolio/>
                    <Route path=format!("{base}/Projects") view=Projects/>
                    <Route path=format!("{base}/Projects/DlcMarkets") view=DlcMarkets/>
                    <Route path=format!("{base}/Projects/LnMarkets") view=LnMarkets/>
                    <Route path=format!("{base}/Projects/SumoRobot") view=SumoRobot/>
                    <Route path=format!("{base}/Projects/Trimble") view=Trimble/>
                    <Route path=format!("{base}/Projects/FFF") view=FFF/>
                    <Route path=format!("{base}/Projects/Allianz") view=Allianz/>
                    <Route path=format!("{base}/Projects/Etl") view=Etl/>
                    <Route path=format!("{base}/Projects/E2ee") view=E2ee/>
                    <Route path=format!("{base}/Games") view=Home/>
                    <Route path=format!("{base}/*") view=NotFound/>
                </Routes>
            </div>
        </Router>
    }
}
