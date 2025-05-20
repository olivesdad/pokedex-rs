use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub weight: i32,
    pub is_default: bool,
    pub sprites: Sprites,
}

impl Pokemon {
    pub fn new() -> Self {
        Pokemon {
            id: -1,
            name: "none".to_owned(),
            weight: -1,
            is_default: false,
            sprites: Sprites::new(),
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct Sprites {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

impl Sprites {
    pub fn new() -> Self {
        Sprites {
            back_default: None,
            back_female: None,
            back_shiny: None,
            back_shiny_female: None,
            front_default: None,
            front_female: None,
            front_shiny: None,
            front_shiny_female: None,
        }
    }
}
