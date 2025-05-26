use api_calls::PokemonIdentifier;

pub mod api_calls;
pub mod data_structures;

#[tokio::main]
async fn main() {
    let poop = api_calls::PokemonIdentifier::IdNumber(12);
    let pee = api_calls::PokemonIdentifier::PokemonName("baxcalibur");
    let peet = api_calls::PokemonIdentifier::PokemonType("ice");
    let data = api_calls::get_pokemon(pee).await.unwrap();
    println!("{:#?}", data);

    let data = api_calls::get_pokemon(peet).await.unwrap();
    println!("{:#?}", data);
}
