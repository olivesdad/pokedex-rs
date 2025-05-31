use core::fmt;


use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PokemonStruct {
    pub id: i32,
    pub name: String,
    pub weight: i32,
    pub is_default: bool,
    pub sprites: Sprites,
    pub types: Vec<PokemonTypes>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonTypes {
    #[serde(alias = "type")]
    pub p_type: PokemonInnerType,
    pub slot: i32,
}
#[derive(Deserialize, Debug)]
pub struct PokemonInnerType {
    pub name: String,
    pub url: String,
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

// trait to default print whatever
impl fmt::Display for PokemonStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut report = format!("\n| Name: {}\n| ID: {}\n| Weight: {}",
                self.name,
                self.id,
                self.weight);
        for (c,t) in self.types.iter().enumerate() {
            report.push_str(&format!(
                "\n| Type{}: {}",
                c+1,
                &t.p_type.name));
        }
        report.push_str("\n\n");
      
        write!(f, "{}", report)
    }
}
