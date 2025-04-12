use crate::external::pokemon_api::PokemonService;
use crate::model::query_root::QueryRoot;
use crate::routes::graphql::{graphql_handler, graphql_playground};
use crate::routes::handler::Handler;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router};
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;

mod external;
mod model;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());
    let handler = Arc::new(Handler { client });
    let client = Arc::new(Client::new());
    let pokemon_servive = PokemonService::new();
    let schema: Schema<QueryRoot, EmptyMutation, EmptySubscription> =
        Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(pokemon_servive) // Add PokemonService
            .data(client.clone()) // Add Client
            .finish();
    let app = Router::new()
        .route("/", get(Handler::check_handler))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/api/{endpoint}", get(Handler::api_handler))
        .layer(Extension(schema))
        .layer(Extension(handler.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Listening on http://{}", addr);
}
