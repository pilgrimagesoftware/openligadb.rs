#[doc = r"* util.rs
 * @paulyhedral
 *
 * Utility functions for making HTTP requests and deserializing JSON responses."]
use serde::de::DeserializeOwned;
use std::error::Error;
use url::Url;

pub(crate) async fn list<M>(url: Url) -> Result<Vec<M>, Box<dyn Error>>
where
    M: DeserializeOwned,
{
    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<M>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

pub(crate) async fn get<M>(url: Url) -> Result<M, Box<dyn Error>>
where
    M: DeserializeOwned,
{
    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json::<M>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}
