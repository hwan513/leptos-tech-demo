use core::result::Result;
use gloo_net::http::Request;
use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;

import_style!(poke_style, "pokedex_list.module.css");

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
#[derive(Clone)]
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
    let (offset, set_offset) = signal(0);
    let limit = 14;
    // Use create_local_resource to fetch Pokémon. This wraps the fetch call and provides helper
    // functions to tell if an async resource is loading / loaded / error
    let pokemons = LocalResource::new(move || fetch_pokemon_species((offset.get(), limit)));
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
            // Very rough pagination. I'm being very lazy here.
            {pokemon_list} <div class=poke_style::buttons>
                // previous and next buttons
                <button
                    disabled=move || offset() == 0
                    on:click=move |_| set_offset.update(|num| *num = (*num - limit).max(0))
                >
                    previous
                </button>
                <button on:click=move |_| {
                    set_offset.update(|num| *num += limit);
                }>next</button>
            </div>
        </aside>
    }
}

/// View for an item in the list
#[component]
fn PokedexItem(pokemon: Pokemon) -> impl IntoView {
    view! {
        <a href=pokemon.id.to_string() class=poke_style::list_item>
            <img src="/images/pokeball.png" />
            <span>{pokemon.name}</span>
        </a>
    }
}
