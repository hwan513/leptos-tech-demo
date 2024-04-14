use leptos::*;
use leptos_router::*;
use stylance::import_style;

use crate::components::pokeshop::{
    calculate_cost::calculate_cost, group_index::group_index, CartContext, Item,
};

import_style!(style, "shopping_cart.module.css");

/// Renders the shopping cart with items, total cost, and checkout button
#[component]
pub fn ShoppingCart() -> impl IntoView {
    view! {
        <div class=style::cart>
            <h3>"Shopping Cart"</h3>
            <CartItems/>
            <A href="checkout">"Checkout"</A>
        </div>
    }
}

/// Renders only the cart items and total cost
#[component]
pub fn CartItems() -> impl IntoView {
    // Clone is used here to allow the closure to take ownership of the items in the cart
    // Another alternative would be to use the `store_value` function which is used in the
    // `CartItem`
    let items = use_context::<Vec<Item>>().unwrap();
    let items_clone = items.clone();
    let cart = use_context::<CartContext>().unwrap().0;
    let groups = move || group_index(cart.get());
    let total_cost =
        move || calculate_cost(cart.get(), items.iter().map(|item| item.cost).collect());

    view! {
        <div class=style::cart_container>
            {items_clone
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    view! { <CartItem count=move || groups()[index] item=item.clone()/> }
                })
                .collect_view()}
        </div>
        <p>"Total cost: 🪙" {total_cost}</p>
    }
}

/// Renders a single item in the cart
#[component]
fn CartItem<F>(count: F, item: Item) -> impl IntoView
where
    F: Fn() -> usize + 'static,
{
    // `store_value` is used as otherwise, either closure will take ownership of the count variable
    // Makes count copyable
    let count = store_value(count);
    view! {
        <Show when=move || { count.with_value(|count| count()) > 0 } fallback=|| view! {}>
            <div class=style::cart_item>
                <img src=&item.image/>
                <span>
                    <strong>{&item.name}</strong>
                </span>
                <span>"× " {count.with_value(|count| count())}</span>
            </div>
        </Show>
    }
}
