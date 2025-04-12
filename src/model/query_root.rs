use async_graphql::{Context, Object};
use reqwest::Client;
use std::sync::Arc;
use std::panic::AssertUnwindSafe;
use futures:: FutureExt;

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


async fn pokemon<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Pokemon {
    match get_dependencies(ctx) {
        Some((service, client)) => {
            AssertUnwindSafe(async move {
                match service.fetch_pokemon(&name, client).await {
                    Ok(pokemon) => pokemon,
                    Err(err) => {
                        eprintln!("Error fetching Pokemon: {:?}", err);
                        Pokemon::no_pokemon()
                    }
                }
            })
            .catch_unwind()
            .await
            .unwrap_or_else(|err| {
                eprintln!("Error fetching Pokemon: {:?}", err);
                Pokemon::no_pokemon()
            })
        }
        None => Pokemon::no_pokemon(),
    }
}

async fn species<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> EvolutionSpecies {
    match get_dependencies(ctx) {
        Some((service, client)) => {
            AssertUnwindSafe(async move {
                match service.fetch_species(&name, client).await{
                    Ok(pokemon_species) => {
                        // println!("POKEMON SERVICE: {:?}", pokemon_species);
                        return pokemon_species;
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        return EvolutionSpecies::no_pokemon();
                    }
                }
            })
            .catch_unwind()
            .await
            .unwrap_or_else(|err| {
                eprintln!("Error fetching Species: {:?}", err);
                EvolutionSpecies::no_pokemon()
            })
        }
        None => EvolutionSpecies::no_pokemon(),
    }
}

async fn evolution<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> EvolutionFlow {
    match get_dependencies(ctx) {
        Some((service, client)) => {
            AssertUnwindSafe(async move {
                match service.fetch_evolution(&name, client).await{
                    Ok(evolution_chain) => evolution_chain,
                    Err(err) => {
                        eprintln!("Error fetching Evolution Chain: {:?}", err);
                        EvolutionFlow::no_evolution_details()
                    }
                }
            })
            .catch_unwind()
            .await
            .unwrap_or_else(|err| {
                eprintln!("Error fetching Evolution Chain: {:?}", err);
                EvolutionFlow::no_evolution_details()
            })
        }
        None => EvolutionFlow::no_evolution_details(),
    }
}
    async fn health(&self, _ctx: &Context<'_>) -> Health {
        Health { is_healthy: true }
    }
}

fn get_dependencies<'ctx>(ctx: &'ctx Context<'_>) -> Option<(&'ctx PokemonService, Arc<Client>)> {
    let service = ctx.data::<PokemonService>();
    let client = ctx.data::<Arc<Client>>();

    match (&service, &client) {
        (Ok(service), Ok(client)) => Some((service, (*client).clone())),
        _ => {
            if service.is_err() {
                eprintln!("PokemonService is not available in the context.");
            }
            if client.is_err() {
                eprintln!("HTTP Client is not available in the context.");
            }
            None
        }
    }
}

// use async_graphql::{Context, Object};
// use reqwest::Client;
// use std::sync::Arc;

// use crate::{
//     external::pokemon_api::PokemonService,
//     routes::{
//         evolution::EvolutionFlow, health::Health, pokemons::Pokemon, species::EvolutionSpecies
//     },
// };

// pub(crate) struct QueryRoot;

// #[Object]
// impl QueryRoot {
//     async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
//         "Hello world"
//     }

//     async fn pokemon<'ctx>(&self, context: &Context<'ctx>, name: String) -> Pokemon {
//         if let (Ok(pokemon_service), Ok(client)) = (context.data::<PokemonService>(), context.data::<Arc<Client>>()) {
//             match pokemon_service.fetch_pokemon(&name, client.clone()).await {
//             Ok(pokemon) => pokemon,
//             Err(err) => {
//                 eprintln!("Error fetching Pokemon: {:?}", err);
//                 Pokemon::no_pokemon()
//             }
//             }
//         } else {
//             if context.data::<PokemonService>().is_err() {
//             eprintln!("PokemonService is not available in the context.");
//             }
//             if context.data::<Arc<Client>>().is_err() {
//             eprintln!("HTTP Client is not available in the context.");
//             }
//             Pokemon::no_pokemon()
//         }
//     }

//     async fn species<'ctx>(&self, context: &Context<'ctx>, name: String)-> EvolutionSpecies {
//         if let(Ok(pokemon_species_service), Ok(client))= (context.data::<PokemonService>(), context.data::<Arc<Client>>()){
//             match pokemon_species_service.fetch_species(&name, client.clone()).await {
//                 Ok(pokemon_species) => {
//                     // println!("POKEMON SERVICE: {:?}", pokemon_species);
//                     return pokemon_species;
//                 }
//                 Err(err) => {
//                     println!("{:?}", err);
//                     return EvolutionSpecies::no_pokemon();
//                 }
//             }
//         } else {
//             if context.data::<PokemonService>().is_err() {
//                 eprintln!("PokemonService is not available in the context.");
//             }
//             if context.data::<Arc<Client>>().is_err() {
//                 eprintln!("HTTP Client is not available in the context.");
//             }
//             return EvolutionSpecies::no_pokemon();
//         }
//     }

//     async fn evolution<'ctx>(&self, context: &Context<'ctx>, name: String) -> EvolutionFlow {
//         if let (Ok(pokemon_service), Ok(client)) = (context.data::<PokemonService>(), context.data::<Arc<Client>>()) {
//             match pokemon_service.fetch_evolution(&name, client.clone()).await {
//                 Ok(evolution_chain) => evolution_chain,
//                 Err(err) => {
//                     eprintln!("Error fetching Evolution Chain: {:?}", err);
//                     EvolutionFlow::no_evolution_details()
//                 }
//             }
//         } else {
//             if context.data::<PokemonService>().is_err() {
//                 eprintln!("PokemonService is not available in the context.");
//             }
//             if context.data::<Arc<Client>>().is_err() {
//                 eprintln!("HTTP Client is not available in the context.");
//             }
//             EvolutionFlow::no_evolution_details()
//         }
//     }

//     async fn health(&self, _ctx: &Context<'_>) -> Health {
//         Health { is_healthy: true }
//     }
// }
