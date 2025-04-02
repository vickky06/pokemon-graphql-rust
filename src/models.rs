use serde::{Deserialize};
use async_graphql::SimpleObject;
#[derive(Debug, SimpleObject, Deserialize)]
pub struct Pokemon{
    pub name: String,
    pub hight: i32,
    pub weight: i32,
    pub base_experience: i32
}