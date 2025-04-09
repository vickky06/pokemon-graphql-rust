use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PokemonApiConfig {
    pub base_uri: String,
    pub pokemon_endpoint: String,
    pub evolution_endpoint: String,
    pub pokemon_species: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub pokemon_api: PokemonApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
}
pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()?;

    settings.try_deserialize::<AppConfig>()
}
