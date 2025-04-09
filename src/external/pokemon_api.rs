use crate::{routes::{evolution::EvolutionFlow, pokemons::Pokemon, species::EvolutionSpecies}};
use axum::{extract::Query, Extension, Json};
use reqwest::{Client, StatusCode};
use std::{collections::HashMap, sync::Arc};

use super::api_caller::{call_api, CallType};

#[derive(Clone,Debug)]
pub struct PokemonService {
    pub client: Arc<Client>
}

impl PokemonService {
    
    pub async fn fetch_pokemon_handler(
        Extension(service): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
        // Path(name): Path<String>,
    ) -> Result<Json<Pokemon>, axum::http::StatusCode> {
        if let Some(name) = params.get("name") {
            match service.fetch_pokemon(name).await {
                Ok(pokemon) => Ok(axum::Json(pokemon)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        } else {
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
    }

    pub async fn fetch_evolution_handler(
        Extension(service): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<EvolutionFlow>, axum::http::StatusCode> {
        if let Some(name) = params.get("name") {
            // println!("{}: -<EVOL", name);
            match service.fetch_evolution(name).await {
                Ok(pokemon) => {
                    // println!("EVOLUTION FLOW OK");
                    let url = &pokemon.evolution_chain.url;
                    println!("EVOLUTION FLOW URL: {:?}", url);
                    match call_api(
                        CallType::GET,
                        url,
                        service.client.clone(),
                        HashMap::from([(String::from("GET"), "".to_string())]),
                    )
                    .await
                    {
                        Ok(evolution_chain) => {
                            let evolution_chain = evolution_chain.json::<EvolutionFlow>().await;
                            match evolution_chain {
                                Ok(flow) => return Ok(axum::Json(flow)),
                                Err(err) => {
                                    println!("Error parsing evolution chain: {:?}", err);
                                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                                }
                            }
                        }
                        Err(e) => {
                            println!("EVOLUTION FLOW NOT OK \n {:?}", e);
                            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                }
                Err(e) => {
                    println!("EVOLUTION FLOW NOT OK \n {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        } else {
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
    }

    pub async fn fetch_pokemon(&self, name: &str) -> Result<Pokemon, reqwest::Error> {
        let response = call_api(
            CallType::GET,
            "fetch_pokemon",
            self.client.clone(),
            HashMap::from([(String::from("GET"), name.to_string())]),
        )
        .await?;
        // Automatically returns a proper `reqwest::Error` if status is not 2xx
        let response = response.error_for_status()?;

        let pokemon = response.json::<Pokemon>().await?;
        Ok(pokemon)
    }

    pub async fn check_handler() -> &'static str {
        "Service is up!"
    }

    pub async fn fetch_evolution(&self, name: &str) -> Result<EvolutionSpecies, reqwest::Error> {
        match call_api(
            CallType::GET,
            "fetch_evolution_chain",
            self.client.clone(),
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
}
