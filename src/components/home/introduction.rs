use leptos::*;

/// Component to show all the features of leptos that has been explored
#[component]
pub fn Introduction() -> impl IntoView {
    view! {
        <section>
            <h1>"Home"</h1>
            <p>
                "This webapp explores the usage of Leptos for client side rendered applications, there are three separate pages: Home, Pokedex and Pokeshop"
            </p>
            <p>
                "Refer to the GitHub readme for setup instructions, and video to see how the code is being used"
            </p>
            <p>"On this page, the following concepts will be explored:"</p>
            <ul>
                // view macro; strings inside tags; no fragments; on:event handlers
                <li>"Basic syntax: JSX ⇒ RSTML; view!{} macro; on:event handling "</li>
                // signal vs state paradigm, create_effect and create_ref also exist
                <li>
                    "Data primitives: " <code>"[state, setState] = useState"</code> "⇒ "
                    <code>"(signal, set_signal) = create_signal()"</code>
                </li>
                <li>
                    "Accessing and modifying data primitives with: "
                    <code>
                        "signal.get(); signal.with(); set_signal.set(); set_signal.update()"
                    </code>
                </li>
                // match statements and show when also exist
                <li>"Simple control flow: Ternary ⇒ if/else statements"</li>
                <li>"Components and props"</li>
                // move || {} closures and derived signals
                <li>"Code execution and callbacks: function ⇒ closures "</li>
                <li>
                    "Iteration: " <code>"array.map()"</code> "⇒"
                    <code>"<For/>; iter.map().collect_view()"</code>
                </li>
                <li>"General styling and scoped CSS with stylance"</li>
            </ul>
            <p>
                "The Pokedex page builds upon what is explored in the home page, while introducing the following concepts"
            </p>
            <ul>
                <li>"Page routing and nested routes" <code>"<Router/>"</code></li>
                <li>"Page layouts and " <code>"<Outlets/>"</code></li>
                <li>"Route params with " <code>"use_params()"</code></li>
                <li>"Code navigation with " <code>"use_navigate()"</code></li>
                <li>"Fetching from an API using " <code>"gloo-net"</code></li>
                <li>"Parsing JSON data with " <code>"Serde"</code></li>
                <li>"Leptos resource handling with " <code>"create_local_resource"</code></li>
            </ul>
            <p>"The Pokeshop builds of what is explored in the other two pages but focuses on:"</p>
            <ul>
                <li>"Global state management with" <code>"use_context"</code></li>
                <li>
                    "Interacting with browser API: "
                    <code>"leptos_use; use_local_storage; web-sys"</code>
                </li>
            </ul>

        </section>
    }
}
