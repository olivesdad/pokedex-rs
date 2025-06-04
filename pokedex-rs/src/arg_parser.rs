use clap::{Parser, Subcommand};

// clap parser bulder thing 
#[derive(Parser, Debug)]
#[command(name = "pokedex-rs")]
#[command(version)]
#[command(about , long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Gets random pokemon
    Random,
    /// get pokemon
    Pokemon {
        identifier: String
    }
}