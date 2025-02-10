use crate::{client::List, constants::API_BASE_URL};
use serde::{ Deserialize, Serialize};
use std::error::Error;
use url::Url;
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sport {
    #[serde(rename(deserialize = "sportId"))]
    pub id: u64,
    #[serde(rename(deserialize = "sportName"))]
    pub name: String,
}

#[async_trait]
impl List for Sport {
    async fn list() -> Result<Vec<Sport>, Box<dyn Error>> {

        let api_url = Url::parse(&format!("{}/getavailablesports", API_BASE_URL))?;

        crate::client::list::<Sport>(api_url).await
    }
}
