use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub height: i32,
    pub weight: i32,
    pub base_experience: i32,
    //  pub evalution_flow: Option<Box<Pokemon>>
}

impl Default for Pokemon {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            height: 0,
            weight: 0,
            base_experience: 0,
        }
    }
}

impl Pokemon {
    pub fn no_pokemon() -> Self {
        let pokemon = Pokemon {
            name: "NO POKEMON FOUND".to_string(),
            height: 0,
            weight: 0,
            base_experience: 0,
            ..Default::default()
        };
        pokemon
    }
}
