use leptos::*;

use crate::components::pokeshop::*;

#[component]
pub fn Pokeshop() -> impl IntoView {
    let items = use_context::<Vec<Item>>().unwrap();

    view! {
        <main>
            <h1>"Pokeshop"</h1>
            <ShoppingCart/>
            {items.iter().map(|item| view! { <Product item=item.clone()/> }).collect_view()}
        </main>
    }
}
