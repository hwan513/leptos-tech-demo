use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::page_layout::PageLayout;
use crate::pages::pokedex::Pokedex;

/// The application component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        // sets the document title
        <Title text="Leptos Demo"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <PageRoutes/>
        </Router>
    }
}

#[component]
fn PageRoutes() -> impl IntoView {
    let navigate = use_navigate();
    let nav_replace = NavigateOptions {
        replace: true,
        ..NavigateOptions::default()
    };
    view! {
        <Routes>
            <Route path="/" view=PageLayout>
                <Route path="" view=move || navigate("home", nav_replace.clone())/>
                <Route path="home" view=Home/>
                <Route path="pokedex" view=Pokedex/>
                <Route path="*" view=NotFound/>
            </Route>
        </Routes>
    }
}
