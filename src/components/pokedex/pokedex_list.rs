use core::result::Result;
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos_router::{
    NavigateOptions,
    hooks::{use_navigate, use_query},
    params::Params,
};
use serde::Deserialize;
use stylance::import_style;

import_style!(poke_style, "pokedex_list.module.css");

#[derive(Params, PartialEq)]
struct OffsetNumber {
    offset: Option<usize>,
}

/// Return type for the pokemon species endpoint
#[derive(Deserialize)]
pub struct PokemonsJson {
    pub results: Vec<PokemonJson>,
}

/// Nested return type for the pokemon endpoint
#[derive(Deserialize)]
pub struct PokemonJson {
    pub name: String,
}

/// Struct used for props passing for `PokedexItem`
#[derive(Clone, Default)]
pub struct Pokemon {
    pub id: usize,
    pub name: String,
}

/// Fetch list of pokemon species from the API and parse the returned data into Vec<Pokemon>
async fn fetch_pokemon_species((offset, limit): (usize, usize)) -> Result<Vec<Pokemon>, Error> {
    let res = Request::get(&format!(
        "https://pokeapi.co/api/v2/pokemon-species?offset={offset}&limit={limit}"
    ))
    .send()
    .await?
    .json::<PokemonsJson>()
    .await?
    .results
    .into_iter()
    .enumerate()
    .map(|(id, pokemon)| Pokemon {
        name: pokemon.name,
        id: id + offset + 1,
    })
    .collect();
    Ok(res)
}

#[component]
pub fn PokedexList() -> impl IntoView {
    // Offset is used to generate IDs for each Pokémon in the list, which the API does not return
    let query = use_query::<OffsetNumber>();
    // This is the most cursed piece of code in existence
    let navigate_prev = use_navigate();
    let navigate_next = use_navigate();
    let nav_options = NavigateOptions {
        replace: true,
        resolve: true,
        ..Default::default()
    };
    let nav_options_clone = nav_options.clone();
    let offset = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|q| q.offset)
            .unwrap_or_default()
    };
    let limit = 14;
    // Use create_local_resource to fetch Pokémon. This wraps the fetch call and provides helper
    // functions to tell if an async resource is loading / loaded / error
    let pokemons = LocalResource::new(move || fetch_pokemon_species((offset(), limit)));
    // `and_then` function will allow the list to display nothing until the resource is loaded
    let pokemon_list = move || {
        pokemons.and_then(|pokemons| {
            pokemons
                .iter()
                .map(|pokemon| {
                    view! { <PokedexItem pokemon=pokemon.clone() /> }
                })
                .collect_view()
        })
    };

    view! {
        <aside class=poke_style::list>
            <Suspense fallback=move || {
                view! {
                    {(0..limit).map(|_| view! { <PokedexItem /> }).collect_view()}
                    <div class=poke_style::buttons>
                        <button disabled>previous</button>
                        <button disabled>next</button>
                    </div>
                }
            }>
                // Very rough pagination. I'm being very lazy here.
                {pokemon_list} <div class=poke_style::buttons>
                    // previous and next buttons
                    <button
                        disabled=move || offset() == 0
                        on:click=move |_| navigate_prev(
                            &format!("?offset={}", (offset() - limit).min(0)),
                            nav_options.clone(),
                        )
                    >
                        previous
                    </button>
                    <button on:click=move |_| navigate_next(
                        &format!("?offset={}", offset() + limit),
                        nav_options_clone.clone(),
                    )>next</button>
                </div>
            </Suspense>
        </aside>
    }
}

/// View for an item in the list
#[component]
fn PokedexItem(#[prop(optional)] pokemon: Pokemon) -> impl IntoView {
    // Check for if the pokemon is as default. i.e. optional
    if pokemon.id == 0 {
        return view! {
            <a href="pokemon" class=poke_style::list_item>
                <img src="/images/pokeball.png" />
                <span class=poke_style::list_item_loading>Loading ...</span>
            </a>
        }
        .into_any();
    }
    view! {
        <a href=pokemon.id.to_string() class=poke_style::list_item>
            <picture>
                <source srcset="/images/pokeball.webp" type="image/webp" />
                <img src="/images/pokeball.png" alt="Pokeball icon" />
            </picture>
            <span>{pokemon.name}</span>
        </a>
    }
    .into_any()
}
