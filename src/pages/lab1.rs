use leptos::*;

use crate::components::lab1::*;

/// This component will encapsulate all the features explored in lab 1
#[component]
pub fn Lab1() -> impl IntoView {
    view! {
        <main>
            <Introduction/>
            <About/>
            <LightBulb/>
        </main>
    }
}
