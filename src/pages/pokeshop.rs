use leptos::*;

use crate::components::pokeshop::Item;
use crate::components::pokeshop::Product;
use crate::components::pokeshop::ShoppingCart;

#[component]
pub fn Pokeshop() -> impl IntoView {
    let items = use_context::<Vec<Item>>().unwrap();

    view! {
        <main>
            <h1>"Pokeshop"</h1>
            <ShoppingCart title="My Cart"/>
            {items.iter().map(|item| view! { <Product item=item.clone()/> }).collect_view()}

        </main>
    }
}
