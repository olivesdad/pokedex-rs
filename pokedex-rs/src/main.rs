use api_calls::PokeReturn;
use viuer::{Config, print};
pub mod api_calls;
pub mod data_structures;

#[tokio::main]
async fn main() {
    //let poop = api_calls::PokemonIdentifier::IdNumber(12);
    let pee = api_calls::PokemonIdentifier::PokemonName("charizard");
    //let peet = api_calls::PokemonIdentifier::PokemonType("ice");
    let data = api_calls::get_pokemon(pee).await.unwrap();
    // println!("{:#?}", data);

    // let data = api_calls::get_pokemon(peet).await.unwrap();
    // println!("{:#?}", data);
    report(data).await;
}

// Take a poke return type and print out the report
pub async fn report(poke_return: PokeReturn) {
    match poke_return {
        //match arm for pokemonStruct
        PokeReturn::ReturnPokemonStruct(x) => {
            // Get image from the sprite thing and print it
            if let Ok(image) = api_calls::get_image(&x).await {
                let (x, y) = if let Ok((x, y)) = crossterm::cursor::position() {
                    (x, y)
                } else {
                    (0, 0)
                };
                // Config for the viu thing
                let conf = Config {
                    // Start from row 4 and column 20.
                    x: x,
                    y: i16::try_from(y).unwrap_or(0),
                    width: Some(50),
                    // height: Some(20),
                    ..Default::default()
                };

                //let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
                print(&image, &conf).expect("Image printing failed.");
            }

            println!("{}", x);
        }
        _ => {}
    }
}
