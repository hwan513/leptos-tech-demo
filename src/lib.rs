use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;
mod structs;

use crate::pages::checkout::Checkout;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::page_layout::PageLayout;
use crate::pages::pokedex_details::PokedexDetails;
use crate::pages::pokedex_layout::PokedexLayout;
use crate::pages::pokeshop::Pokeshop;
use crate::pages::pokeshop_layout::PokeshopLayout;

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

#[component(transparent)]
fn PageRoutes() -> impl IntoView {
    // Only way to reuse the replace due to rust borrow checker
    let nav_replace = NavigateOptions {
        replace: true,
        ..Default::default()
    };
    let nav_replace_clone = nav_replace.clone();

    view! {
        <Routes>
            <Route path="/" view=PageLayout>
                <Route path="" view=move || use_navigate()("home", nav_replace.clone())/>
                <Route path="home" view=Home/>
                <Route path="pokedex" view=PokedexLayout>
                    <Route
                        path=""
                        view=move || use_navigate()("pokedex/1", nav_replace_clone.clone())
                    />

                    <Route path=":id" view=PokedexDetails/>
                </Route>
                <Route path="pokeshop" view=PokeshopLayout>
                    <Route path="" view=Pokeshop/>
                    <Route path="checkout" view=Checkout/>
                </Route>
                <Route path="*" view=NotFound/>
            </Route>
        </Routes>
    }
}
