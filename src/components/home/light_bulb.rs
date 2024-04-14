use leptos::*;

/// LightBulb component, with an on:click event to toggle the light between on and off
/// Usage of using if/else for control flow is demonstrated here
#[component]
pub fn LightBulb() -> impl IntoView {
    let (is_on, set_on) = create_signal(false);
    // Closures can also be set to variables which can then be called later on with on:click
    let toggle_light = move |_| {
        set_on(!is_on.get());
    };
    view! {
        <section>
            <img
                // To keep reactitivity, the if/else control flow needs to enclosed by a closure:
                // move || if {} else {}
                src=move || if is_on() { "images/light-on.png" } else { "images/light-off.png" }
                on:click=toggle_light
            />
        </section>
    }
}
