use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename(deserialize = "teamId"))]
    pub id: i32,
    #[serde(rename(deserialize = "teamName"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "shortName"))]
    pub short_name: Option<String>,
    #[serde(rename(deserialize = "teamIconUrl"))]
    pub icon_url: Option<String>,
    #[serde(rename(deserialize = "teamGroupName"))]
    pub group: Option<String>,
}

impl Team {
    async fn available(league: &str, season: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getavailableteams/{}/{}",
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

    async fn last_match(league: &str, id: i32) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyleagueteam/{}/{}",
            API_BASE_URL, league, id
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
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
    async fn test_available_teams() {
        let league = "bl1";
        let season = 2024;
        let teams: Result<Vec<Team>, Box<dyn Error>> = Team::available(league, season).await;
        dbg!(&teams);

        assert!(teams.is_ok());
    }

    #[actix_web::test]
    async fn test_last_match() {
        let league = "bl1";
        let team_id = 2024;
        let team: Result<Team, Box<dyn Error>> = Team::last_match(league, team_id).await;
        dbg!(&team);

        assert!(team.is_ok());
    }
}
