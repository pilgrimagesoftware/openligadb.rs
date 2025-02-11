/// Utility functions.
/// @paulyhedral
use reqwest;
use serde::de::DeserializeOwned;
use std::error::Error;
use url::Url;

/// Returns a list of deserialized objects from the API.
pub async fn list<M>(url: Url) -> Result<Vec<M>, Box<dyn Error>> where M : DeserializeOwned + 'static {
    let response = reqwest::get(url.as_str())
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<M>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

/// Returns a single deserialized object from the API.
pub async fn get<M>(url: Url) -> Result<M, Box<dyn Error>> where M : DeserializeOwned + 'static {
    let response = reqwest::get(url.as_str())
        .await
        .map_err(|e| e.to_string())?
        .json::<M>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}
