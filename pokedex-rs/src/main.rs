use api_calls::PokeReturn;
use viuer::{Config, print};
pub mod api_calls;
pub mod data_structures;
pub mod arg_parser;
use arg_parser::Args;
use clap::Parser;
use rand::Rng;




#[tokio::main]
async fn main() {
    // use parser
    let args = Args::parse();

    // Just get the str out of the arg for lookup. it should be a pokemon name or id number
    // Should probably refactor later if adding new options
    let mut lookup_val = "1".to_owned();

    if let Some(ref x) = args.command {
    match x {
        arg_parser::Commands::Pokemon{identifier: pokemon} => {
            lookup_val = pokemon.clone();
        }
        arg_parser::Commands::Random => {
            let num = rand::rng().random_range(1..=1025);
            lookup_val = num.to_string();
        }
    }
}
    let pee = api_calls::PokemonIdentifier::PokemonName(&lookup_val);
    let data = api_calls::get_pokemon(pee).await;

    if let Ok(x) = data {
        report(x).await;
    } else {
        println!("Failed to locate requested pokemon record")
    }
}

// Take a poke return type and print out the report
pub async fn report(poke_return: PokeReturn) {
    match poke_return {
        //match arm for pokemonStruct
        PokeReturn::ReturnPokemonStruct(x) => {
            // Get image from the sprite thing and print it
            if let Ok(image) = api_calls::get_image(&x).await {
                // find cursor position so we know where to print
                let (x, y) = if let Ok((x, y)) = crossterm::cursor::position() {
                    (x, y)
                } else {
                    (0, 0)
                }; // get cursor location 

                // attempt to get terminal width or default to 40
                let c = if let Ok((c, _)) = crossterm::terminal::size() {
                    c
                } else {
                    40
                };

                // Config for the viu thing
                let conf = Config {
                    // print location for sprite
                    x: x,
                    y: i16::try_from(y).unwrap_or(0),
                    use_iterm: true,
                    // we only care about how wide it is because terminal can scroll for height and it will scale ok
                    width: Some(u32::try_from(c).unwrap_or(40)),

                    ..Default::default()
                };

                print(&image, &conf).expect("Image printing failed.");
            } // iflet on image

            // Summar from pokemonstruct
            println!("{}", x);
        } // matchest pokemon struct return type
        _ => {}
    }
}
