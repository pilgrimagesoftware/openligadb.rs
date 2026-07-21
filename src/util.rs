#[doc = r"* util.rs
 * @paulyhedral
 *
 * Utility functions for making HTTP requests and deserializing JSON responses."]
use crate::error::OpenLigaError;
use serde::de::DeserializeOwned;
use url::Url;

pub(crate) async fn list<M>(url: Url) -> Result<Vec<M>, OpenLigaError>
where
    M: DeserializeOwned,
{
    let response = reqwest::get(url).await?.json::<Vec<M>>().await?;

    Ok(response)
}

pub(crate) async fn get<M>(url: Url) -> Result<M, OpenLigaError>
where
    M: DeserializeOwned,
{
    let response = reqwest::get(url).await?.json::<M>().await?;

    Ok(response)
}
