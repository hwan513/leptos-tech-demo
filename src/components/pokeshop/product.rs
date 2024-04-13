use leptos::*;
use stylance::import_style;

use super::pokeshop_context::Item;
use crate::components::pokeshop::CartContext;

import_style!(style, "product.module.css");

#[component]
pub fn Product(item: Item) -> impl IntoView {
    let CartContext(_, set_cart) = use_context::<CartContext>().unwrap();
    view! {
        <div class=style::product>
            <img src=item.image/>
            <div>
                <h3>{item.name}</h3>
                <p>"Cost:" {item.cost}</p>
                <button on:click=move |_| {
                    set_cart.update(|cart| cart.push(item.id));
                }>"Add to cart"</button>
            </div>
        </div>
    }
}
