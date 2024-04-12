use leptos::*;
use leptos_router::*;

#[component]
pub fn PageLayout() -> impl IntoView {
    view! {
        <nav>
            <a>"Home"</a>
            <a>"Pokedex"</a>
            <a>"PokeShop"</a>
        </nav>
        <div class="container">
            <Outlet/>
        </div>
    }
}
