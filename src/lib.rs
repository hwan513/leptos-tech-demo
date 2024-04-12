use leptos::*;
use leptos_meta::*;

mod components;
mod pages;

use pages::lab1::Lab1;

/// The application component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        // sets the document title
        <Title text="Leptos Demo"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <div class="container">
            <Lab1/>
        </div>
    }
}
