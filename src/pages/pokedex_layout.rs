use leptos::*;
use leptos_router::*;
use stylance::import_style;

use crate::components::pokedex::PokedexList;

import_style!(poke_style, "pokedex_layout.module.css");

#[component]
pub fn PokedexLayout() -> impl IntoView {
    view! {
        <h1>"Pokedex"</h1>
        <div class=poke_style::dex_container>
            <PokedexList/>
            <main class=poke_style::dex_view>
                <Outlet/>
            </main>
        </div>
    }
}
