use leptos::*;
use leptos_router::*;

#[component]
pub fn PageLayout() -> impl IntoView {
    view! {
        <nav>
            <a href="/home">"Home"</a>
            <a href="/pokedex">"Pokedex"</a>
            <a href="/pokeshop">"PokeShop"</a>
        </nav>
        <div class="container">
            <Outlet/>
        </div>
    }
}
