use leptos::*;
use leptos_router::*;

use crate::components::pokeshop::PokeshopContextProvider;

#[component]
pub fn PokeshopLayout() -> impl IntoView {
    view! {
        <PokeshopContextProvider>
            <Outlet/>
        </PokeshopContextProvider>
    }
}
