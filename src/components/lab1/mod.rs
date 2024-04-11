use leptos::*;

#[component]
pub fn Lab1() -> impl IntoView {
    view! {
        <h1>"Lab 1"</h1>
        <About/>
    }
}

#[component]
pub fn About() -> impl IntoView {
    let (name, setName) = create_signal(String::new());
    view! {
        <input
            type="email"
            placeholder="Enter your name"
            on:input=move |ev| {
                setName(event_target_value(&ev));
            }
        />

        <AboutMe name=name age=8/>
    }
}

#[component]
pub fn AboutMe(
    name: ReadSignal<String>,
    #[prop(optional)] age: u8,
    #[prop(default="pizza".to_string())] favourite_food: String,
) -> impl IntoView {
    view! {
        <p>"Hello " {name} " welcome to Leptos Demo!"</p>
        <p>"My name is Henry. I'm " {age} " years old, and I like " {favourite_food} "."</p>
    }
}

#[component]
pub fn LightBulb() -> impl IntoView {
    let (is_on, set_on) = create_signal(false);
    let toggle_light = move |_| {
        set_on(!is_on.get());
    };
    view! {
        <img
            src=move || if is_on() { "images/light-on.png" } else { "images/light-off.png" }
            on:click=toggle_light
        />
    }
}
