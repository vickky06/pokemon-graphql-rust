use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct EvolutionFlow {
    pub baby_trigger_item: Option<serde_json::Value>, // Can be null or an object
    pub chain: Chain,
    pub id: u32,
    pub evolution_matrix : Option<Vec<Vec<String>>>, // Placeholder for unknown structure that needs to be manually generated
}

#[derive(Serialize, Deserialize, Debug, SimpleObject,Clone)]
pub struct Chain {
    pub evolution_details: Vec<EvolutionDetail>,
    pub evolves_to: Vec<Chain>,
    pub is_baby: bool,
    pub species: Species,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject,Clone)]
pub struct EvolutionDetail {
    pub gender: Option<u8>,
    pub held_item: Option<Item>,
    pub item: Option<Item>,
    pub known_move: Option<serde_json::Value>, // Placeholder for unknown structure
    pub known_move_type: Option<serde_json::Value>, // Placeholder for unknown structure
    pub location: Option<serde_json::Value>,   // Placeholder for unknown structure
    pub min_affection: Option<u8>,
    pub min_beauty: Option<u8>,
    pub min_happiness: Option<u8>,
    pub min_level: Option<u8>,
    pub needs_overworld_rain: bool,
    pub party_species: Option<serde_json::Value>, // Placeholder for unknown structure
    pub party_type: Option<serde_json::Value>,    // Placeholder for unknown structure
    pub relative_physical_stats: Option<i8>,
    pub time_of_day: String,
    pub trade_species: Option<serde_json::Value>, // Placeholder for unknown structure
    pub trigger: Trigger,
    pub turn_upside_down: bool,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject,Clone)]
pub struct Item {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject,Clone)]
pub struct Trigger {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
pub struct Species {
    pub name: String,
    pub url: String,
}

impl EvolutionFlow {
    pub fn no_evolution_details() -> Self {
        let evo = EvolutionFlow::default();
        evo
    }
    pub fn default() -> Self {
        Self {
            baby_trigger_item: None,
            chain: Chain::default(),
            id: 0,
            evolution_matrix: None, // Placeholder for unknown structure
        }
    }
}
impl Chain {
    pub fn default() -> Self {
        Self {
            evolution_details: Vec::new(),
            evolves_to: Vec::new(),
            is_baby: false,
            species: Species::default(),
        }
    }
}
impl Species {
    pub fn default() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
        }
    }
}
