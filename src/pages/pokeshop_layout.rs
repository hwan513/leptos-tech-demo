use leptos::*;
use leptos_router::*;

use crate::components::pokeshop::PokeshopContextProvider;

/// Wraps the pages within the Pokeshop layout with the pokeshop context
#[component]
pub fn PokeshopLayout() -> impl IntoView {
    view! {
        <PokeshopContextProvider>
            <Outlet/>
        </PokeshopContextProvider>
    }
}
