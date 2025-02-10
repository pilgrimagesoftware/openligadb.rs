use reqwest;
use serde::de::DeserializeOwned;
use std::error::Error;
use url::Url;
use async_trait::async_trait;

pub struct Client {}

#[async_trait]
pub trait List<M> {
    async fn list() -> Result<Vec<M>, Box<dyn Error>>;
}

pub trait Get {
    async fn get<M>() -> Result<M, Box<dyn Error>>;
}

pub async fn list<M>(url: Url) -> Result<Vec<M>, Box<dyn Error>> where M : DeserializeOwned + 'static {
    let response = reqwest::get(url.as_str())
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<M>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

pub async fn get<M>(url: Url) -> Result<M, Box<dyn Error>> where M : DeserializeOwned + 'static {
    let response = reqwest::get(url.as_str())
        .await
        .map_err(|e| e.to_string())?
        .json::<M>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}
