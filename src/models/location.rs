#![doc = r"The Location object"]
use serde::{Deserialize, Serialize};

/// A data structure representing a match location
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    /// The identifier of this location data
    #[serde(rename(deserialize = "locationID"))]
    pub id: i32,
    /// The city where the match took place
    #[serde(rename(deserialize = "locationCity"))]
    pub city: Option<String>,
    /// The stadium where the match took place
    #[serde(rename(deserialize = "locationStadium"))]
    pub stadium: Option<String>,
}
