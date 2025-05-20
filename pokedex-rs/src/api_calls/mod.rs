use crate::data_structures::pokemon::PokemonStruct;
use reqwest::get;
use std::fmt::format;

//use crate::data_structures;

pub enum PokemonIdentifier<'a> {
    IdNumber(i32),
    PokemonName(&'a str),
}

pub async fn get_pokemon<'a>(identifier: PokemonIdentifier<'a>) -> Result<PokemonStruct, ()> {
    match identifier {
        PokemonIdentifier::IdNumber(id) => {
            let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
            let resp = get(url).await.unwrap().text().await.unwrap();
            let data: PokemonStruct = serde_json::from_str(&resp).unwrap();
            return Ok(data);
        }
        PokemonIdentifier::PokemonName(name) => {
            // Make call to api with name
            let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
            let resp = get(url).await.unwrap().text().await.unwrap();
            let data: PokemonStruct = serde_json::from_str(&resp).unwrap();
            return Ok(data);
        }
    }
   Err(())
}
