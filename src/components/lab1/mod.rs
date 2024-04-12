use leptos::*;

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

/// Usage of signals and prop is demonstrated here
#[component]
pub fn About() -> impl IntoView {
    let (name, setName) = create_signal(String::new());
    view! {
        <section>
            <input
                type="email"
                placeholder="Enter your name"
                on:input=move |ev| {
                    setName(event_target_value(&ev));
                }
            />

            <AboutMe name=name age=21/>
        </section>
    }
}

#[component]
pub fn AboutMe(
    /// This is a required component property, the code will not compile without it
    /// You can also see how ReadSignal<> can be passed into props by defining the type
    name: ReadSignal<String>,
    /// Optional property: defaults to the default value of the data type: 0 for u8
    #[prop(optional)]
    age: u8,
    /// Optional property with default: if no value is given, then it will default to pizza
    #[prop(default="pizza".to_string())]
    favourite_food: String,
) -> impl IntoView {
    view! {
        <p>"Hello " {name} "and welcome to Leptos Demo!"</p>
        <p>"My name is Henry. I'm " {age} " years old, and I like " {favourite_food} "."</p>
    }
}

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
