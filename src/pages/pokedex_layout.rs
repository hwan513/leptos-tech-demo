use leptos::prelude::*;
use leptos_router::components::Outlet;
use stylance::import_style;

use crate::components::pokedex::PokedexList;

import_style!(poke_style, "pokedex_layout.module.css");

/// Pokedex layout, sets up the pokedex list side menu and the pokedex view
#[component]
pub fn PokedexLayout() -> impl IntoView {
    let value = use_context::<RwSignal<i32>>();
    view! {
        <h1>"Pokedex"</h1>
        <p>{value}</p>
        <div class=poke_style::dex_container>
            <PokedexList />
            <main class=poke_style::dex_view>
                <Outlet />
            </main>
        </div>
    }
}
