use crate::data_structures::{
    pokemon::PokemonStruct, pokemon_species::PokemonSpeciesStruct, pokemon_types::PokemonTypeInfo,
};
use anyhow::Result;
use image::{DynamicImage, load_from_memory};
use reqwest::get;

pub enum PokemonIdentifier<'a> {
    IdNumber(i32),
    PokemonName(&'a str),
    PokemonSpecies(&'a str),
    PokemonType(&'a str),
}

impl<'a> PokemonIdentifier<'a> {
    pub fn get_url(&self) -> String {
        match self {
            Self::IdNumber(x) => {
                format!("https://pokeapi.co/api/v2/pokemon/{}", x)
            }
            Self::PokemonName(name) => {
                format!("https://pokeapi.co/api/v2/pokemon/{}", name)
            }
            Self::PokemonSpecies(name) => {
                format!("https://pokeapi.co/api/v2/pokemon-species/{}", name)
            }
            Self::PokemonType(name) => {
                format!("https://pokeapi.co/api/v2/type/{}", name)
            }
        }
    }
}

// enums to wrap the different return types from the get_pokemon call
#[derive(Debug)]
pub enum PokeReturn {
    ReturnPokemonStruct(PokemonStruct),
    ReturnPokemonSpeciesStruct(PokemonSpeciesStruct),
    ReturnTypeStruct(PokemonTypeInfo),
}

// hit various apis and deserialize return struct
pub async fn get_pokemon<'a>(identifier: PokemonIdentifier<'a>) -> Result<PokeReturn> {
    // GEt the url and then get json string
    let url = &identifier.get_url();
    let resp = get(url).await?.text().await?;

    // Deserialize to the proper struct
    match identifier {
        PokemonIdentifier::IdNumber(_) | PokemonIdentifier::PokemonName(_) => {
            let data: PokemonStruct = serde_json::from_str(&resp)?;
            return Ok(PokeReturn::ReturnPokemonStruct(data));
        }
        PokemonIdentifier::PokemonSpecies(_) => {
            // data: PokemonStruct = serde_json::from_str(&resp).unwrap();
            return Err(anyhow::anyhow!("Not implimented yet"));
        }
        PokemonIdentifier::PokemonType(_) => {
            let data: PokemonTypeInfo = serde_json::from_str(&resp)?;
            return Ok(PokeReturn::ReturnTypeStruct(data));
        }
    }
}

// should return a dynamic image. N
pub async fn get_image(pokemon_struct: &PokemonStruct) -> Result<DynamicImage> {
    if let Some(x) = &pokemon_struct.sprites.front_default {
        let img_bytes = get(x).await?.bytes().await?;
        let img = load_from_memory(&img_bytes);
        if img.is_ok() {
            return Ok(img.unwrap());
        }
    }
    Err(anyhow::anyhow!("failed to get image"))
}
