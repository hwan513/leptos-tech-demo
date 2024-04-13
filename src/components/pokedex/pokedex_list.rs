use gloo_net::http::Request;
use leptos::{error::Result, *};
use leptos_router::*;
use stylance::import_style;

use crate::structs::pokemon::*;

import_style!(poke_style, "pokedex_list.module.css");

async fn fetch_pokemon_species((offset, limit): (usize, usize)) -> Result<Vec<Pokemon>> {
    let res = Request::get(&format!("https://pokeapi.co/api/v2/pokemon-species?offset={offset}&limit={limit}"))
        .send()
        .await?
        .json::<PokemonsJson>()
        .await?.results.into_iter().enumerate().map(|(id,pokemon)| Pokemon {
            name: pokemon.name,
            id: id + offset + 1,
        })
        .collect();
    Ok(res)
}

#[component]
pub fn PokedexList() -> impl IntoView {
    // offset is used to generate IDs for each pokemon in the list, which the API does not return
    let (offset, set_offset) = create_signal(0);
    let limit = 14;
    let pokemons = create_local_resource(move || (offset.get(), limit), fetch_pokemon_species);
    let pokemon_list = move || {
        pokemons.and_then(|pokemons| {
            pokemons
                .iter()
                .map(|pokemon| {
                    view! { <PokedexItem pokemon=pokemon.clone()/> }
                })
                .collect_view()
        })
    };

    view! {
        <aside class=poke_style::list>
            // Very rough pagination. I'm being very lazy here.
            {pokemon_list} <div class=poke_style::buttons>
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

#[component]
fn PokedexItem(pokemon: Pokemon) -> impl IntoView {
    view! {
        <A href=pokemon.id.to_string() class=poke_style::list_item>
            <img src="images/pokeball.png"/>
            <span>{pokemon.name}</span>
        </A>
    }
}

