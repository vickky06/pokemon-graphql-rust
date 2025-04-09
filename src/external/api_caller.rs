use super::config_loader::{load_config, AppConfig};
use reqwest::{Client, Response};
// use serde_json;
use std::{collections::HashMap, sync::Arc};
// use warp::reply::Json;

pub enum CallType {
    GET,
    // POST,
}

struct EndpointType {
    config: Result<AppConfig, config::ConfigError>,
}

impl EndpointType {
    fn new() -> Self {
        Self {
            config: load_config(),
        }
    }
    pub(crate) fn get_pokemon_endpoint(self, endpoint_name: &str) -> String {
        let config = self.config.unwrap();
        match endpoint_name {
            "fetch_pokemon" => format!(
                "{}/{}",
                config.api.pokemon_api.base_uri, config.api.pokemon_api.pokemon_endpoint
            ),
            "fetch_evolution" => format!(
                "{}/{}",
                config.api.pokemon_api.base_uri, config.api.pokemon_api.evolution_endpoint
            ),
            "fetch_evolution_chain" => format!(
                "{}/{}",
                config.api.pokemon_api.base_uri, config.api.pokemon_api.pokemon_species
            ),
            _ => format!(
                "{}/{}",
                config.api.pokemon_api.base_uri, config.api.pokemon_api.pokemon_endpoint
            ),
        }
    }
}

pub async fn call_api(
    call_type: CallType,
    endpoint_name: &str,
    client: Arc<Client>,
    args: HashMap<String, String>,
) -> Result<Response, reqwest::Error> {
    let endpoint_loader = EndpointType::new();
    let response: Response;
    // println!("Calling external api");
    let mut endpoint = match (endpoint_name, &call_type) {
        ("fetch_pokemon", CallType::GET) => endpoint_loader.get_pokemon_endpoint(endpoint_name),
        ("fetch_evolution", CallType::GET) => endpoint_loader.get_pokemon_endpoint(endpoint_name),
        ("fetch_evolution_chain", CallType::GET) => {
            endpoint_loader.get_pokemon_endpoint(endpoint_name)
        }
        _ => {
            // endpoint is url
            endpoint_name.to_string()
        }
    };
    match call_type {
        CallType::GET => {
            endpoint.push_str("/");
            // endpoint.push_str(&args["GET"]);
            // println!("{}: -<", endpoint);
            match args.get("GET") {
                Some(value) => {
                    endpoint.push_str(value);
                }
                None => {}
            }
            response = client.get(&endpoint).send().await?;
        } // CallType::POST => {
          //     unimplemented!()
          // }
    }
    // println!("{:?}",response);
    Ok(response)
}
