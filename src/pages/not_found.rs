use leptos::*;
use leptos_router::use_location;

#[component]
pub fn NotFound() -> impl IntoView {
    let location = use_location();
    view! {
        <h1>"Page not found"</h1>
        <p>"The page you tried to access " {location.pathname} " does not exist"</p>
    }
}
