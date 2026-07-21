#![doc = r"The Group object and methods"]
#[cfg(feature = "http-client")]
use crate::constants::API_BASE_URL;
#[cfg(feature = "http-client")]
use crate::error::OpenLigaError;
use serde::{Deserialize, Serialize};
#[cfg(feature = "http-client")]
use url::Url;

/// A data structure representing a group
#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    /// The identifier of this group data
    #[serde(rename(deserialize = "groupID"))]
    pub id: i32,
    /// The name of the group
    #[serde(rename(deserialize = "groupName"))]
    pub name: Option<String>,
    /// The order identifier of the group
    #[serde(rename(deserialize = "groupOrderID"))]
    pub order_id: i32,
}

#[cfg(feature = "http-client")]
impl Group {
    /// Get the current league group
    ///
    /// Fetches the current group for a specific league.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    pub async fn current(league: &str) -> Result<Self, OpenLigaError> {
        let api_url = Url::parse(&format!("{}/getcurrentgroup/{}", API_BASE_URL, league))?;

        let response = reqwest::get(api_url.as_str()).await?.json::<Self>().await?;

        Ok(response)
    }

    /// A list of groups for a league and season
    ///
    /// Fetches a list of groups for the specified league and season.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    pub async fn available(league: &str, season: i32) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getavailablegroups/{}/{}",
            API_BASE_URL, league, season
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await?
            .json::<Vec<Self>>()
            .await?;

        Ok(response)
    }
}

#[cfg(all(test, feature = "http-client"))]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_available() {
        let league = "bl1";
        let season = 2024;
        let groups: Result<Vec<Group>, OpenLigaError> = Group::available(league, season).await;
        dbg!(&groups);

        assert!(groups.is_ok());
    }

    #[actix_web::test]
    async fn test_current() {
        let league = "bl1";
        let group: Result<Group, OpenLigaError> = Group::current(league).await;
        dbg!(&group);

        assert!(group.is_ok());
    }
}
