use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct EvolutionSpecies {
    pub name: String,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub evolution_chain: EvolutionChain,
}
#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
}

#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct Language {
    pub name: String,
}

#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct EvolutionChain {
    pub url: String,
}

impl Default for EvolutionSpecies {
    fn default() -> Self {
        Self {
            name: String::new(),
            flavor_text_entries: Vec::new(),
            evolution_chain: EvolutionChain { url: String::new() },
        }
    }
}

impl EvolutionSpecies {
    pub async fn evolution_flow() -> Self {
        let evo_species = EvolutionSpecies::default();
        evo_species
    }
}
