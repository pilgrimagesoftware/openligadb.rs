use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename(deserialize = "groupID"))]
    pub id: i32,
    #[serde(rename(deserialize = "groupName"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "groupOrderID"))]
    pub order_id: i32,
}
