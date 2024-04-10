use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // sets the document title
        // injects metadata in the <head> of the page

        <Router>
            <Routes>
            <Route path="" view=Home/>
            <Route path="404" view=NotFound/>
            </Routes>
        </Router>
    }
}
