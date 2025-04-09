// use axum::{response::IntoResponse, Json};
// use reqwest::StatusCode;
use async_graphql::SimpleObject;

use serde::Deserialize;

#[derive(Debug, Deserialize, SimpleObject)]
pub struct Health {
    pub is_healthy: bool,
}

impl Health{
    pub fn new(is_healthy: bool) -> Self {
        Health { is_healthy:true }
    }
}