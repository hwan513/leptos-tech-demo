use leptos::*;

#[component]
pub fn Introduction() -> impl IntoView {
    view! {
        <section>
            <h1>"Home"</h1>
            <p>"This page will explore the features explored in lab 1 including"</p>
            <ul>
                // view macro; strings inside tags; no fragments; on:event
                <li>"Basic syntax: JSX ⇒ RSTML"</li>
                // signal vs state paradigm, create_effect and create_ref also exist
                <li>"Data primitives: useState ⇒ create_signal"</li>
                // match statements and show when also exist
                <li>"Simple control flow: Ternary ⇒ if/else statements"</li>
                <li>"Components and props"</li>
                <li>"Code execution and callbacks: function ⇒ closures "</li>
                <li>"Iteration: .map() ⇒ <For/> "</li>
            </ul>
            <p>"Navigate to Pokedex..."</p>
            <p>"Navigate to Pokeshop..."</p>
        </section>
    }
}
