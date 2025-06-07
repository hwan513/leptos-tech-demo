use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// This page gets called if the user tries to access a page that does not exist
#[component]
pub fn NotFound() -> impl IntoView {
    let location = use_location();
    view! {
        <h1>"Page not found"</h1>
        <p>"The page you tried to access " {location.pathname} " does not exist"</p>
    }
}
