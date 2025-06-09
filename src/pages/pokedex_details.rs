use core::result::Result;
use gloo_net::http::Request;
use leptos::Params;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use serde::Deserialize;
use stylance::import_style;

/// Params for `PokedexDetails` to use with `use_params`
#[derive(Params, PartialEq, Clone)]
struct PokedexParams {
    id: usize,
}

/// Struct used to deserialize API json response
#[derive(Clone, Deserialize)]
struct PokemonDetails {
    name: String,
    id: usize,
    flavor_text_entries: Vec<FlavorTexts>,
}

/// Nested struct used to deserialize JSON API response
#[derive(Clone, Deserialize)]
struct FlavorTexts {
    flavor_text: String,
}

import_style!(styles, "pokedex_details.module.css");

/// Fetches pokemon details from API based on id
async fn fetch_pokemon_details(id: usize) -> Result<PokemonDetails, Error> {
    let res = Request::get(&format!("https://pokeapi.co/api/v2/pokemon-species/{id}"))
        .send()
        .await?
        .json::<PokemonDetails>()
        .await?;
    Ok(res)
}

/// Fetches pokemon details from API based on id and then selects which component to render based
/// on result
#[component]
pub fn PokedexDetails() -> impl IntoView {
    let params = use_params::<PokedexParams>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let pokemon = LocalResource::new(move || fetch_pokemon_details(id()));

    view! {
        <Suspense fallback=move || {
            view! { <PokemonLoading /> }.into_any()
        }>
            {move || match pokemon.get() {
                None => view! { <PokemonLoading /> }.into_any(),
                Some(result) => {
                    match result {
                        Ok(pokemon) => view! { <PokemonPage pokemon=pokemon /> }.into_any(),
                        Err(_) => view! { <PokemonNotFound id=id() /> }.into_any(),
                    }
                }
            }}
        </Suspense>
    }
}

/// View for pokemon details page with pokemon image and description
#[component]
fn PokemonPage(pokemon: PokemonDetails) -> impl IntoView {
    // Sanitise string as API returns strings with the form control character
    // Strings might not be in English since I can't be bothered switching filtering by English
    let (is_img_loading, set_is_img_loading) = signal(true);
    let (img_style, set_img_style) = signal(String::from("display:none"));
    let description = pokemon
        .flavor_text_entries
        .first()
        .unwrap()
        .flavor_text
        .replace('', " ")
        .clone();
    view! {
        <h2>{pokemon.name}</h2>
        <img
            class=styles::dex_image
            src=format!(
                "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/home/{}.png",
                pokemon.id,
            )
            style=img_style
            on:load=move |_| {
                set_img_style(String::new());
                set_is_img_loading(false);
            }
        />
        <Show when=move || is_img_loading()>
            <PlaceholderImage />
        </Show>

        <p>{description}</p>
    }
}

/// View for when the API response returns an error
#[component]
fn PokemonNotFound(id: usize) -> impl IntoView {
    view! {
        <h2>"Pokemon not found"</h2>
        <img class=styles::dex_image src="/white-pokeball.png" />
        <p>"Unfortunately, a pokemon with id " <code>{id}</code> " is not registered in the dex"</p>
    }
}

/// View for when the API response is still loading, this is called less often as the
/// `<Suspense/>` component is not being used
#[component]
fn PokemonLoading() -> impl IntoView {
    view! {
        <h1>Loading...</h1>
        <PlaceholderImage />
        <p>We are currently loading...</p>
    }
}

/// View for the placeholder image
#[component]
fn PlaceholderImage() -> impl IntoView {
    view! {
        <picture>
            <source srcset="/images/white-pokeball.webp" type="image/webp" />
            <img
                class=styles::placeholder_image
                src="/images/white-pokeball.png"
                alt="Placeholder Pokeball"
            />
        </picture>
    }
}
