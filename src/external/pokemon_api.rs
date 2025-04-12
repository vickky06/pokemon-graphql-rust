use crate::{routes::{evolution::EvolutionFlow, pokemons::Pokemon, species::EvolutionSpecies}, utils::flattern::flatten_evolution_chain_iterative};
// use axum::{extract::Query, Extension, Json};
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};

use super::api_caller::{call_api, CallType};

#[derive(Clone, Debug, Copy)]
pub struct PokemonService {}

impl PokemonService {
    pub fn new() -> Self {
        PokemonService {}
    }
    pub async fn fetch_pokemon(
        self,
        name: &str,
        client: Arc<Client>,
    ) -> Result<Pokemon, reqwest::Error> {
        let response = call_api(
            CallType::GET,
            "fetch_pokemon",
            client,
            HashMap::from([(String::from("GET"), name.to_string())]),
        )
        .await?;
        // Automatically returns a proper `reqwest::Error` if status is not 2xx
        let response = response.error_for_status()?;

        let pokemon = response.json::<Pokemon>().await?;
        Ok(pokemon)
    }

    pub async fn fetch_species(
        self,
        name: &str,
        client: Arc<Client>,
    ) -> Result<EvolutionSpecies, reqwest::Error> {
        match call_api(
            CallType::GET,
            "fetch_evolution_chain",
            client,
            HashMap::from([(String::from("GET"), name.to_string())]),
        )
        .await
        {
            Ok(pokemon) => {
                let pokemon = pokemon.json::<EvolutionSpecies>().await?;

                return Ok(pokemon);
            }
            Err(e) => {
                println!("EVOLUTION FLOW NOT OK \n {:?}", e);
                return Err(e);
            }
        }
    }

    pub async fn fetch_evolution(
        self,
        name: &str,
        client: Arc<Client>,
    ) -> Result<EvolutionFlow, reqwest::Error> {
        match call_api(
            CallType::GET,
            "fetch_evolution_chain",
            client.clone(),
            HashMap::from([(String::from("GET"), name.to_string())]),
        )
        .await
        {
            Ok(pokemon_evo) => {
                let pokemon_evo = pokemon_evo.json::<EvolutionSpecies>().await?;
                let url = &pokemon_evo.evolution_chain.url;
                match call_api(CallType::GET, url, client, HashMap::new()).await {
                    Ok(res) => {
                        let parsed = res.json::<EvolutionFlow>().await;
                        match parsed {
                            Ok(mut flow) => {
                                let evolution_matrix =
                                flatten_evolution_chain_iterative(
                                    flow.clone().chain,
                                );
                            flow.evolution_matrix = Some(evolution_matrix);
                                return Ok(flow)
                            },
                            Err(e) => return Err(e),
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(e) => {
                println!("EVOLUTION FLOW NOT OK");
                return Err(e);
            }
        }
    }
}
