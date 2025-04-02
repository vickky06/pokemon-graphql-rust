use crate::models::Pokemon;
use reqwest::Client;
use std::sync::Arc;

// TODO : need to use to get from env file using dotenv crate
const POKEMON_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";

#[derive(Clone)]
pub struct PokemonService {
    client: Arc<Client>,
}

impl PokemonService {
/// Creates a new instance of `PokemonService` with an HTTP client.
/// 
/// # Returns
/// 
/// A `PokemonService` instance with a new `reqwest::Client` wrapped in an `Arc` for shared ownership.

    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
        }
    }

    pub async fn fetch_pokemon(&self, name: &str) -> Result<Pokemon, reqwest::Error> {
        let url = format!("{}/{}", POKEMON_API_BASE_URL, name);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }
}
