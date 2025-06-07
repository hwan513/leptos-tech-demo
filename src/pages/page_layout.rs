use leptos::prelude::*;
use leptos_router::components::{Outlet, A};

/// This page sets up the navigation bar and wraps the content of the page with class container
#[component]
pub fn PageLayout() -> impl IntoView {
    view! {
        <nav>
            <A href="/home">"Home"</A>
            <A href="/pokedex">"Pokedex"</A>
            <A href="/pokeshop">"Pokeshop"</A>
        </nav>
        <div class="container">
            <Outlet />
        </div>
    }
}
