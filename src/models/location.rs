use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename(deserialize = "locationID"))]
    pub id: i32,
    #[serde(rename(deserialize = "locationCity"))]
    pub city: Option<String>,
    #[serde(rename(deserialize = "locationStadium"))]
    pub stadium: Option<String>,
}
