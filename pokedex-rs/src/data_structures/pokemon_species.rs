use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PokemonSpeciesStruct {
    pub id: i32,
    pub name: String,
    pub weight: i32,
    pub is_default: bool,
   // pub sprites: Sprites,
    //pub types: Vec<PokemonTypes>,
}