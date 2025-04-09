use crate::routes::{evolution::EvolutionFlow, pokemons::Pokemon, species::EvolutionSpecies};
use axum::{
    extract::{Path, Query},
    Extension,
};
use reqwest::{Client, StatusCode};
use std::{collections::HashMap, sync::Arc};

use super::api_caller::{call_api, CallType};


pub struct PokemonService {
    pub client: Arc<Client>,
}
impl PokemonService {
    pub async fn fetch_pokemon_handler(
        Extension(service): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
        // Path(name): Path<String>,
    ) -> Result<String, axum::http::StatusCode> {
        if let Some(name) = params.get("name") {
            match service.fetch_pokemon(name).await {
                Ok(pokemon) => Ok(format!("Fetched Pokémon: {:?}", pokemon)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        } else {
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
    }

    pub async fn fetch_evolution_handler(
        Extension(service): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<String, axum::http::StatusCode> {

        if let Some(name) = params.get("name") {
            // println!("{}: -<EVOL", name);
            match service.fetch_evolution(name).await {
                Ok(pokemon) => {
                    // println!("EVOLUTION FLOW OK");
                    println!("{:?}", pokemon);
                    Ok(format!("Fetched Pokémon Evolution: {:?}", pokemon))
                },
                Err(e) =>{
                    println!("EVOLUTION FLOW NOT OK \n {:?}",e);
                     Err(StatusCode::INTERNAL_SERVER_ERROR)
                    },
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
                println!("EVOLUTION FLOW NOT OK \n {:?}",e);
                return Err(e)
            },
        }
    }

    // pub async fn get_pokemon_handler(
    //     Extension(service): Extension<Arc<PokemonService>>,
    //     Query(params): Query<HashMap<String, String>>,
    //     Path(endpoint): Path<String>,
    // ) -> Result<String, axum::http::StatusCode> {
    //     println!("{}: -<", endpoint);
    //     match endpoint.as_str() {
    //         "fetch_pokemon" => {
    //             if let Some(name) = params.get("name") {
    //                 match service.fetch_pokemon(name).await {
    //                     Ok(pokemon) => {
    //                         let pokemon = Pokemon {
    //                             name: pokemon.name,
    //                             height: pokemon.height,
    //                             weight: pokemon.weight,
    //                             base_experience: pokemon.base_experience,
    //                             ..Default::default()
    //                         };
    //                         // let evolution_chain: Option<Vec<Pokemon>>=
    //                         Ok(format!("Fetched Pokeomon: {:?}", pokemon))
    //                     }
    //                     Err(e) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    //                 }
    //             } else {
    //                 Err(axum::http::StatusCode::BAD_REQUEST)
    //             }
    //         }
    //         "fetch_evolution" => {
    //             if let Some(name) = params.get("name") {
    //                 match service.fetch_evolution(name).await {
    //                     Ok(pokemon_evo) => {
    //                         // let evo_url = pokemon.url;

    //                         Ok(format!("Fetched Evolutio n: {:?}", pokemon_evo))
    //                     }
    //                     Err(e) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    //                 }
    //             } else {
    //                 Err(axum::http::StatusCode::BAD_REQUEST)
    //             }
    //         }
    //         "" => Ok(format!("Hello World")),
    //         _ => Err(axum::http::StatusCode::NOT_FOUND),
    //     }
    // }
}
