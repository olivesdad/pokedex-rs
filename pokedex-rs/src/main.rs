use api_calls::{PokeReturn};
//use data_structures::pokemon::PokemonStruct;

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

    // let data = api_calls::get_pokemon(poop).await.unwrap();
    // println!("{:#?}", &data);

    if let PokeReturn::ReturnPokemonStruct(ref x) = data {
        if let Ok(image) = api_calls::get_image(
            &x, //"https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/shiny/998.png",
        )
        .await
        {
            use viuer::{Config, print};

            let conf = Config {
                // Start from row 4 and column 20.
                x: 50,
                y: 0,
               // width: Some(50),
               // height: Some(20),

               // restore_cursor: true,
                ..Default::default()
            };

            //let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
            print(&image, &conf).expect("Image printing failed.");
        }
    }
    if let PokeReturn::ReturnPokemonStruct(x) = data {

    println!("{}", x)
    }
}
