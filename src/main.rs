use crate::external::pokemon_api::PokemonService;
use crate::model::query_root::QueryRoot;
use crate::routes::graphql::{graphql_handler, graphql_playground};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router};
// use external::config_loader::{load_config};
use reqwest::Client;
use std::{net::SocketAddr, sync::Arc};

mod external;
mod model;
mod routes;

#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());
    let pokemon_service = Arc::new(PokemonService { client });
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let app = Router::new()
        .route("/", get(PokemonService::check_handler))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        // .route("/check",get(PokemonService::check_handler()))
        .route(
            "/debug/fetch_pokemon",
            get(PokemonService::fetch_pokemon_handler),
        )
        .route(
            "/debug/fetch_evolution",
            get(PokemonService::fetch_evolution_handler),
        )
        // .route(
        //     "/debug/{*endpoint}",
        //     get(PokemonService::get_pokemon_handler),
        // ) // TODO : convert this to internal endpoint
        .layer(Extension(schema))
        .layer(Extension(pokemon_service));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Listening on http://{}", addr);
}
