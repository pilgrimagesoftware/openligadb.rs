#![doc = r"The Team object and methods"]
#[cfg(feature = "http-client")]
use crate::constants::API_BASE_URL;
#[cfg(feature = "http-client")]
use crate::util;
use serde::{Deserialize, Serialize};
#[cfg(feature = "http-client")]
use std::error::Error;
#[cfg(feature = "http-client")]
use url::Url;

/// A data structure representing a team
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    /// The identifier of the team data
    #[serde(rename(deserialize = "teamId"))]
    pub id: i32,
    /// The full proper name of the team
    #[serde(rename(deserialize = "teamName"))]
    pub name: Option<String>,
    /// A short, familiar name for the team
    #[serde(rename(deserialize = "shortName"))]
    pub short_name: Option<String>,
    /// A URL for the team's logo or icon
    #[serde(rename(deserialize = "teamIconUrl"))]
    pub icon_url: Option<String>,
    /// The group name for the team
    #[serde(rename(deserialize = "teamGroupName"))]
    pub group: Option<String>,
}

#[cfg(feature = "http-client")]
impl Team {
    /// Get available teams
    ///
    /// Fetches a list of the available teams for a given league and season.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    pub async fn available(league: &str, season: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getavailableteams/{}/{}",
            API_BASE_URL, league, season
        ))?;

        util::list::<Self>(api_url).await
    }
}

#[cfg(all(test, feature = "http-client"))]
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
