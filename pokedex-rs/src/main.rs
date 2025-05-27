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

   let image = api_calls::get_image("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/shiny/998.png").await.unwrap();
   //println!("{:#?}", image);

    use viuer::{print, Config};

let conf = Config {
    // Start from row 4 and column 20.
    x: 50,
    y: 0,
    ..Default::default()
};

//let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
print(&image, &conf).expect("Image printing failed.");
}
