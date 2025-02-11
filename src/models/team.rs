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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const LEIPZIG_TEAM_ID: i32 = 1635;
    const BUNDESLIGA_ID: i32 = 4741;
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
}
