use leptos::*;

#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub cost: usize,
    pub image: String,
}

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

#[derive(Clone, Copy, Debug)]
pub struct CartContext(pub RwSignal<Vec<usize>>);

impl CartContext {
    pub fn new() -> Self {
        Self(create_rw_signal(vec![]))
    }
}

#[component]
pub fn PokeshopContextProvider(children: Children) -> impl IntoView {
    provide_context(CartContext::new());
    provide_context(initial_products());

    view! { {children()} }
}
