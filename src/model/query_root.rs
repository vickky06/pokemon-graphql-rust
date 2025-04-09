use async_graphql::{Context, Object};

use crate::routes::{evolution::EvolutionFlow, health::Health, pokemons::Pokemon, species::EvolutionSpecies};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    // (3)
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello world"
    }

    async fn health(&self, _ctx: &Context<'_>) -> Health {
        return Health { is_healthy: true };
    }

    async fn pokemon(&self, _ctx: &Context<'_>) -> Pokemon {
        Pokemon::pokemon().await
    }

    async fn pokemon_species(&self, _ctx: &Context<'_>) -> EvolutionSpecies {
        EvolutionSpecies::evolution_flow().await
    }
    async fn pokemon_evalution(&self, _ctx: &Context<'_>) -> EvolutionFlow {
        EvolutionFlow::evolution_details().await
    }
}
