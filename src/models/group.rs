//! The Group object and methods
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

impl Group {
    async fn current(
        league: &str
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getcurrentgroup/{}",
            API_BASE_URL, league
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn available(
        league: &str,
        season: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getavailablegroups/{}/{}",
            API_BASE_URL, league, season
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Vec<Self>>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[actix_web::test]
    async fn test_available() {
        let league = "bl1";
        let season = 2024;
        let groups: Result<Vec<Group>, Box<dyn Error>> = Group::available(league, season).await;
        dbg!(&groups);

        assert!(groups.is_ok());
    }

    #[actix_web::test]
    async fn test_current() {
        let league = "bl1";
        let group: Result<Group, Box<dyn Error>> = Group::current(league).await;
        dbg!(&group);

        assert!(group.is_ok());
    }
}
