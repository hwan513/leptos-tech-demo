use serde::Deserialize;

#[derive(Deserialize)]
pub struct PokemonsJson {
    pub results: Vec<PokemonJson>,
}

#[derive(Deserialize)]
pub struct PokemonJson {
    pub name: String,
}

#[derive(Clone)]
pub struct Pokemon {
    pub id: usize,
    pub name: String,
}
