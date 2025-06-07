use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Html, Meta, Title};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::{hooks::use_navigate, path, NavigateOptions};

mod components;
mod pages;

use crate::pages::{
    Checkout, Home, NotFound, PageLayout, PokedexDetails, PokedexLayout, Pokeshop, PokeshopLayout,
};

/// The application component
#[allow(clippy::must_use_candidate)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let nav_replace = NavigateOptions {
        replace: true,
        resolve: true,
        ..Default::default()
    };
    let nav_replace_clone = nav_replace.clone();
    view! {
        <Html {..} lang="en" dir="ltr" attr:data-theme="light" />
        // sets the document title
        <Title text="Leptos Demo" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Router>
            <Routes fallback=|| "Not found.">
                <ParentRoute path=path!("/") view=PageLayout>
                    <Route
                        path=path!("")
                        view=move || use_navigate()("home", nav_replace.clone())
                    />
                    <Route path=path!("home") view=Home />
                    <ParentRoute path=path!("pokedex") view=PokedexLayout>
                        <Route
                            path=path!("")
                            view=move || use_navigate()("1", nav_replace_clone.clone())
                        />

                        <Route path=path!(":id") view=PokedexDetails />
                    </ParentRoute>
                    <ParentRoute path=path!("pokeshop") view=PokeshopLayout>
                        <Route path=path!("") view=Pokeshop />
                        <Route path=path!("checkout") view=Checkout />
                    </ParentRoute>
                    <Route path=path!("*") view=NotFound />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
