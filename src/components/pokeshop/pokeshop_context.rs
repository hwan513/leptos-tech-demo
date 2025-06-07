use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;

/// An item in the pokeshop
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub cost: usize,
    pub image: String,
}

/// The initial set of items in the pokeshop
fn initial_products() -> Vec<Item> {
    vec![
        Item {
            id: 0,
            name: "Abra".to_string(),
            cost: 180,
            image: "/images/Abra.png".to_string(),
        },
        Item {
            id: 1,
            name: "Clefairy".to_string(),
            cost: 500,
            image: "/images/Clefairy.png".to_string(),
        },
        Item {
            id: 2,
            name: "Nidorina".to_string(),
            cost: 1200,
            image: "/images/Nidorina.png".to_string(),
        },
        Item {
            id: 3,
            name: "Dratini".to_string(),
            cost: 2800,
            image: "/images/Dratini.png".to_string(),
        },
        Item {
            id: 4,
            name: "Scyther".to_string(),
            cost: 5500,
            image: "/images/Scyther.png".to_string(),
        },
        Item {
            id: 5,
            name: "Porygon".to_string(),
            cost: 9999,
            image: "/images/Porygon.png".to_string(),
        },
    ]
}

/// The context struct for the pokeshop
#[derive(Clone, Copy, Debug)]
pub struct CartContext(pub Signal<Vec<usize>>, pub WriteSignal<Vec<usize>>);

/// The `new` impl allows for the use of a signal that writes to the local storage using `use_local_storage`.
impl CartContext {
    pub fn new() -> Self {
        let (state, set_state, _) =
            use_local_storage::<Vec<usize>, JsonSerdeCodec>("shopping-cart");
        Self(state, set_state)
    }
}

// braces required for the view macro to function correctly; this allow block stops compiler warnings
#[allow(unused_braces)]
/// A context provider for the pokeshop, wraps the children and provides context that can be called
/// with `use_context::<ContextStruct>()`
#[component]
pub fn PokeshopContextProvider(children: Children) -> impl IntoView {
    provide_context(CartContext::new());
    provide_context(initial_products());

    view! { {children()} }
}
