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

    async fn last_match(league_id: i32, id: i32) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyleagueteam/{}/{}",
            API_BASE_URL, league_id, id
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

    const LEIPZIG_TEAM_ID: i32 = 1635;
    const BUNDESLIGA_ID: i32 = 3;
    const BUNDESLIGA: &str = "bl1";

    #[actix_web::test]
    async fn test_available_teams() {
        let league = BUNDESLIGA;
        let season = 2024;
        let result: Result<Vec<Team>, Box<dyn Error>> = Team::available(league, season).await;
        dbg!(&result);

        assert!(result.is_ok());
        let teams = result.unwrap();
        assert_eq!(teams.len(), 18); // 18 teams in Bundesliga
    }

    #[actix_web::test]
    async fn test_last_match() {
        let league = BUNDESLIGA_ID;
        let team_id = LEIPZIG_TEAM_ID;
        let result: Result<Team, Box<dyn Error>> = Team::last_match(league, team_id).await;
        dbg!(&result);

        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team.id, LEIPZIG_TEAM_ID);
    }
}
