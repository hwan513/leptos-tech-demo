use leptos::*;

#[component]
pub fn Introduction() -> impl IntoView {
    view! {
        <section>
            <h1>"Lab 1"</h1>
            <p>"This section will explore the features explored in lab 1 including"</p>
            <ul>
                // view macro; strings inside tags; no fragments
                <li>"Basic syntax: JSX ⇒ RSTML"</li>
                // signal vs state paradigm, create_effect and create_ref also exist
                <li>"Data primitives: useState ⇒ create_signal"</li>
                // match statements and show when also exist
                <li>"Simple control flow: Ternary ⇒ if/else statements"</li>
                <li>"Components and props"</li>
                <li>"Code execution: function ⇒ closures "</li>
            </ul>
        </section>
    }
}
