use crate::models::sport::Sport;
use crate::{client::List, constants::API_BASE_URL};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename(deserialize = "locationID"))]
    pub id: i32,
    #[serde(rename(deserialize = "locationCity"))]
    pub city: Option<String>,
    #[serde(rename(deserialize = "locationStadium"))]
    pub stadium: Option<String>,
}
