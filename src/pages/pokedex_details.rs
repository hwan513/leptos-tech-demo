use gloo_net::http::Request;
use leptos::{error::Result, *};
use leptos_router::*;
use serde::Deserialize;
use stylance::import_style;

#[derive(Params, PartialEq)]
struct PokedexParams {
    id: usize,
}

#[derive(Clone, Deserialize)]
struct PokemonDetails {
    name: String,
    id: usize,
    flavor_text_entries: Vec<FlavorTexts>,
}

#[derive(Clone, Deserialize)]
struct FlavorTexts {
    flavor_text: String,
}

import_style!(styles, "pokedex_details.module.css");

async fn fetch_pokemon_details(id: usize) -> Result<PokemonDetails> {
    let res = Request::get(&format!("https://pokeapi.co/api/v2/pokemon-species/{id}"))
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

    let pokemon = create_local_resource(id, fetch_pokemon_details);

    // Suspense fallback can be added if you want to see a loading screen between clicks
    view! {
        {move || match pokemon.get() {
            None => view! { <PokemonLoading/> },
            Some(result) => {
                match result {
                    Ok(pokemon) => {
                        view! { <PokemonPage pokemon=pokemon/> }
                    }
                    Err(_) => view! { <PokemonNotFound id=id()/> },
                }
            }
        }}
    }
}

#[component]
fn PokemonPage(pokemon: PokemonDetails) -> impl IntoView {
    // Sanitise string as  api returns strings with the form control character
    // Strings might not be in english since I can't be bothered switching filtering by english
    let description = &pokemon.flavor_text_entries.first().unwrap().flavor_text.replace('', " ");
    view! {
        <h2>{pokemon.name}</h2>
        <img
            class=styles::dex_image
            src=format!(
                "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/home/{}.png",
                pokemon.id,
            )
        />

        <p>{description}</p>
    }
}

#[component]
fn PokemonNotFound(id: usize) -> impl IntoView {
    view! {
        <h2>"Pokemon not found"</h2>
        <img class=styles::dex_image src="/white-pokeball.png"/>
        <p>"Unfortunately, a pokemon with id " <code>{id}</code> " is not registered in the dex"</p>
    }
}

#[component]
fn PokemonLoading() -> impl IntoView {
    view! {
        <h1>Loading...</h1>
        <PlaceholderImage/>
        <p>We are currently loading...</p>
    }
}

#[component]
fn PlaceholderImage() -> impl IntoView {
    view! { <img class=styles::placeholder_image src="/images/white-pokeball.png"/> }
}
