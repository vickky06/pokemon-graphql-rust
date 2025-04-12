use crate::external::api_caller::{call_api, CallType};
use crate::external::pokemon_api::PokemonService;
use crate::routes::{evolution::EvolutionFlow, pokemons::Pokemon, species::EvolutionSpecies};
use crate::utils::flattern::flatten_evolution_chain_iterative;
use axum::extract::Path;
use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Extension, Json,
};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Debug)]
pub struct Handler {
    pub client: Arc<Client>,
}
impl Handler {
    // pub fn new(client: Arc<Client>) -> Self {
    //     Handler { client }
    // }

    pub async fn api_handler(
        Path(endpoint): Path<String>,
        Query(params): Query<HashMap<String, String>>,
        // Extension(service): Extension<Arc<PokemonService>>,
    ) -> Response {
        let client = Arc::new(Client::new());

        match endpoint.as_str() {
            "fetch_evolution" => {
                if let Some(name) = params.get("name") {
                    match PokemonService::new()
                        .fetch_species(name, client.clone())
                        .await
                    {
                        Ok(pokemon) => {
                            let url = &pokemon.evolution_chain.url;
                            println!("EVOLUTION FLOW URL: {:?}", url);
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
                                            return Json(flow).into_response();
                                        }
                                        Err(_) => {
                                            return StatusCode::INTERNAL_SERVER_ERROR
                                                .into_response()
                                        }
                                    }
                                }
                                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                            }
                        }
                        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                    }
                }
            }
            "fetch_pokemon" => {
                if let Some(name) = params.get("name") {
                    match PokemonService::new()
                        .fetch_pokemon(name, client.clone())
                        .await
                    {
                        Ok(pokemon) => return Json(pokemon).into_response(),
                        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                    }
                }
            }
            "fetch_species" => {
                if let Some(name) = params.get("name") {
                    match PokemonService::new()
                        .fetch_species(name, client.clone())
                        .await
                    {
                        Ok(species) => return Json(species).into_response(),
                        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                    }
                }
            }
            _ => {
                return Json(json!({ "message": "Unknown endpoint" })).into_response();
            }
        }

        StatusCode::BAD_REQUEST.into_response()
    }

    pub async fn fetch_evolution_handler(
        Extension(_): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<EvolutionFlow>, axum::http::StatusCode> {
        let client = Arc::new(Client::new());
        if let Some(name) = params.get("name") {
            match PokemonService::new()
                .fetch_species(name, client.clone())
                .await
            {
                Ok(pokemon) => {
                    let client = client.clone();
                    // println!("EVOLUTION FLOW OK");
                    let url = &pokemon.evolution_chain.url;
                    println!("EVOLUTION FLOW URL: {:?}", url);
                    match call_api(
                        CallType::GET,
                        url,
                        client,
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

    pub async fn fetch_pokemon_handler(
        Extension(_): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<Pokemon>, axum::http::StatusCode> {
        let client = Arc::new(Client::new());
        if let Some(name) = params.get("name") {
            match PokemonService::new()
                .fetch_pokemon(name, client.clone())
                .await
            {
                Ok(pokemon) => {
                    // println!("POKEMON FLOW OK");
                    return Ok(axum::Json(pokemon));
                }
                Err(e) => {
                    println!("POKEMON FLOW NOT OK \n {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        } else {
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
    }


    pub async fn fetch_species_handler(
        Extension(_): Extension<Arc<PokemonService>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<EvolutionSpecies>, axum::http::StatusCode> {
        let client = Arc::new(Client::new());
        if let Some(name) = params.get("name") {
            match PokemonService::new()
                .fetch_species(name, client.clone())
                .await
            {
                Ok(pokemon) => {
                    // println!("POKEMON FLOW OK");
                    return Ok(axum::Json(pokemon));
                }
                Err(e) => {
                    println!("POKEMON FLOW NOT OK \n {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        } else {
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
    }

    pub async fn check_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
        if let Some(value) = params.get("key") {
            format!("Query parameter 'key' has value: {}", value)
        } else {
            "No query parameter 'key' found".to_string()
        }
    }
}
