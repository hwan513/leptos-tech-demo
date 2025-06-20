use leptos::prelude::*;

use crate::components::home::{About, Introduction, LightBulb, TodoList};

/// This component will encapsulate all the features explored in lab 1
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Introduction />
            <About />
            <LightBulb />
            <TodoList />
        </main>
    }
}
