use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PokemonTypeInfo {
    pub name: String,
    pub damage_relations: DamageRelations,
}

#[derive(Deserialize, Debug)]
pub struct PokemonType {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug)]
pub struct DamageRelations {
    pub double_damage_from: Vec<PokemonType>,
    pub double_damage_to: Vec<PokemonType>,
    pub half_damage_from: Vec<PokemonType>,
    pub half_damage_to: Vec<PokemonType>,
    pub no_damage_from: Vec<PokemonType>,
    pub no_damage_to: Vec<PokemonType>,
}