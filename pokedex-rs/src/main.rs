pub mod api_calls;
pub mod data_structures;
use data_structures::pokemon::Pokemon;

#[tokio::main]
async fn main() {
    let poop = api_calls::PokemonIdentifier::IdNumber(12);
    let pee = api_calls::PokemonIdentifier::PokemonName("pikachu");
    let data = api_calls::get_pokemon(pee).await.unwrap();
    println!("{:#?}", data);
}
