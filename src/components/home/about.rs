use chrono::{Datelike, Local, NaiveDate};
use leptos::prelude::*;

/// About component, with an on:input event to set the name for the greeting and a brief
/// introduction of myself.
/// Usage of signals and prop (optional and default) is demonstrated here.
#[component]
pub fn About() -> impl IntoView {
    let (name, setName) = signal(String::new());
    let birthday = NaiveDate::from_ymd_opt(2002, 10, 22).unwrap();
    let now = Local::now().date_naive();
    let mut age = now.year() - birthday.year();
    // Account dates within each year
    if now.ordinal0() < birthday.ordinal0() {
        age -= 1;
    }

    view! {
        <section>
            <input
                type="email"
                placeholder="Enter your name"
                on:input=move |ev| {
                    setName(event_target_value(&ev));
                }
            />
            <AboutMe name=name age=age />
        </section>
    }
}

#[component]
fn AboutMe(
    /// This is a required component property, the code will not compile without it
    /// You can also see how ReadSignal<> can be passed into props by defining the type
    name: ReadSignal<String>,
    /// Optional property: defaults to the default value of the data type.
    #[prop(optional)]
    age: i32,
    /// Optional property with default: if no value is given, then it will default to pizza
    #[prop(default="pizza 🍕".to_string())]
    favourite_food: String,
) -> impl IntoView {
    view! {
        <p>"Hello " {name} " and welcome to Leptos Demo!"</p>
        <p>"My name is Henry. I'm " {age} " years old, and I like " {favourite_food} "."</p>
    }
}
