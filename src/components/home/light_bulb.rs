use leptos::prelude::*;

/// LightBulb component, with an on:click event to toggle the light between on and off
/// Usage of using if/else for control flow is demonstrated here
#[component]
pub fn LightBulb() -> impl IntoView {
    let (is_on, set_on) = signal(false);
    // Closures can also be set to variables which can then be called later on with on:click
    let toggle_light = move |_| {
        set_on(!is_on.get());
    };
    // To keep reactivity, the if/else control flow needs to be enclosed by a closure:
    // move || if {} else {}
    let image_source = move || {
        if is_on() {
            "images/light-on"
        } else {
            "images/light-off"
        }
    };
    view! {
        <section>
            <p>{"Click to toggle the light bulb"}</p>
            <picture>
                // `move || code()` is additionally also needed here for reactivity
                <source srcset=move || format!("{}.webp", image_source()) type="image/webp" />
                <img src=move || format!("{}.png", image_source()) on:click=toggle_light />
            </picture>
        </section>
    }
}
