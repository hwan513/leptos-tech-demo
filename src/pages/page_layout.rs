use leptos::*;
use leptos_router::*;

#[component]
pub fn PageLayout() -> impl IntoView {
    view! {
        <nav>
            <A href="/home">"Home"</A>
            <A href="/pokedex">"Pokedex"</A>
            <A href="/pokeshop">"Pokeshop"</A>
        </nav>
        <div class="container">
            <Outlet/>
        </div>
    }
}
