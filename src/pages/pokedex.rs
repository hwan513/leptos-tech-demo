use leptos::*;
use leptos_router::*;
use stylance::import_style;

import_style!(poke_style, "pokedex.module.css");

#[component]
pub fn Pokedex() -> impl IntoView {
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

#[component]
fn PokedexList() -> impl IntoView {
    view! { <aside class=poke_style::list>"Text"</aside> }
}
