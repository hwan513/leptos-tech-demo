use gloo_net::http::Request;
use leptos::{error::Result, *};
use leptos_router::*;
use serde::Deserialize;

#[derive(Params, PartialEq)]
struct PokedexParams {
    id: usize,
}

#[derive(Deserialize)]
struct PokemonDetails {
    name: String,
}

async fn fetch_pokemon_details((): ()) -> Result<PokemonDetails> {
    let res = Request::get(&format!("https://pokeapi.co/api/v2/pokemon-species/1"))
        .send()
        .await?
        .json::<PokemonDetails>()
        .await?;
    Ok(res)
}

#[component]
pub fn PokedexDetails() -> impl IntoView {
    let params = use_params::<PokedexParams>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let pokemons = create_local_resource(move || (), fetch_pokemon_details);

    view! { <p>{id}</p> }
}
