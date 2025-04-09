use async_graphql::{Context, Object};

use crate::{
    external::pokemon_api::PokemonService,
    routes::{
        evolution::EvolutionFlow, health::Health, pokemons::Pokemon, species::EvolutionSpecies,
    },
};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello world"
    }

    async fn pokemon<'ctx>(&self, context: &Context<'ctx>, name: String) -> Pokemon {
        // let pokemon_service:&PokemonService = ctx.data().unwrap();
        match context.data::<PokemonService>(){
            Ok(pokemon_service) => {
                println!("{}",name);
                println!("POKEMON SERVICE: {:?}", pokemon_service);
            },
            Err(_) => {
                println!("POKEMON SERVICE: None");
            }
        };
        Pokemon::no_pokemon()
    }

    async fn pokemon_species(&self, _ctx: &Context<'_>) -> EvolutionSpecies {
        EvolutionSpecies::evolution_flow().await
    }
    async fn pokemon_evalution(&self, _ctx: &Context<'_>) -> EvolutionFlow {
        EvolutionFlow::evolution_details().await
    }

    async fn health(&self, _ctx: &Context<'_>) -> Health {
        Health { is_healthy: true }
    }
}
