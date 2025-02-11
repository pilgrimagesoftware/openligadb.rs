use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;
use crate::util;

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

        util::list::<Self>(api_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

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
