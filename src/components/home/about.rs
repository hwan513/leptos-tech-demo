use leptos::*;

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
fn AboutMe(
    /// This is a required component property, the code will not compile without it
    /// You can also see how ReadSignal<> can be passed into props by defining the type
    name: ReadSignal<String>,
    /// Optional property: defaults to the default value of the data type: 0 for u8
    #[prop(optional)]
    age: u8,
    /// Optional property with default: if no value is given, then it will default to pizza
    #[prop(default="pizza 🍕".to_string())]
    favourite_food: String,
) -> impl IntoView {
    view! {
        <p>"Hello " {name} " and welcome to Leptos Demo!"</p>
        <p>"My name is Henry. I'm " {age} " years old, and I like " {favourite_food} "."</p>
    }
}
