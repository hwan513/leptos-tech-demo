use leptos::*;

use crate::components::pokeshop::{CartContext, ShoppingCart};

#[component]
pub fn Checkout() -> impl IntoView {
    let cart = use_context::<CartContext>().unwrap().0;
    view! {
        <main>
            <h1>"Checkout"</h1>
            <ShoppingCart title="Checkout"/>
        </main>
    }
}
