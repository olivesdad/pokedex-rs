use crate::data_structures::pokemon::PokemonStruct;
use reqwest::get;
use std::fmt::format;

//use crate::data_structures;

pub enum PokemonIdentifier<'a> {
    IdNumber(i32),
    PokemonName(&'a str),
}

impl<'a> PokemonIdentifier<'a> {
    pub fn get_url(self) -> String {
        match self {
            Self::IdNumber(x) => {
                format!("https://pokeapi.co/api/v2/pokemon/{}", x)
            }
            Self::PokemonName(name) => {
                format!("https://pokeapi.co/api/v2/pokemon/{}", name)
            }
        }
    }
}

pub async fn get_pokemon<'a>(identifier: PokemonIdentifier<'a>) -> Result<PokemonStruct, ()> {
    match identifier {
        PokemonIdentifier::IdNumber(_) | PokemonIdentifier::PokemonName(_) => {
            let url = identifier.get_url();
            let resp = get(url).await.unwrap().text().await.unwrap();
            let data: PokemonStruct = serde_json::from_str(&resp).unwrap();
            return Ok(data);
        }
    }
    Err(())
}
