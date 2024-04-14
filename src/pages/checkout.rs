use leptos::*;

use crate::components::pokeshop::{CartContext, CartItems};

/// Checkout page for the pokeshop. It will clear the cart on purchase
#[component]
pub fn Checkout() -> impl IntoView {
    let CartContext(cart, set_cart) = use_context::<CartContext>().unwrap();
    view! {
        <main>
            <h1>"Checkout"</h1>
            {if cart.with(std::vec::Vec::is_empty) {
                view! {
                    <h4>"Cart is empty"</h4>
                    <a href="/pokeshop">"Back"</a>
                }
                    .into_view()
            } else {
                view! {
                    <CartItems/>
                    <a href="/pokeshop" on:click=move |_| set_cart.update(std::vec::Vec::clear)>
                        "Checkout"
                    </a>
                }
                    .into_view()
            }}

        </main>
    }
}
