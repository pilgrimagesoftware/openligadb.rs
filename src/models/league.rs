//! The League object and methods
use crate::constants::API_BASE_URL;
use crate::models::sport::Sport;
use crate::util;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

/// A data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    /// The identifier of this league data
    #[serde(rename(deserialize = "leagueId"))]
    pub id: i32,
    /// The name of the league
    #[serde(rename(deserialize = "leagueName"))]
    pub name: Option<String>,
    /// The shortcut value for the league. This is used for some other API calls.
    #[serde(rename(deserialize = "leagueShortcut"))]
    pub shortcut: Option<String>,
    /// The season for this league
    #[serde(rename(deserialize = "leagueSeason"))]
    pub season: Option<String>,
    /// The sport to which this league belongs
    pub sport: Sport,
}

impl League {
    /// List the leagues
    ///
    /// Fetches a list of leagues.
    pub async fn list() -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!("{}/getavailableleagues", API_BASE_URL))?;

        util::list::<Self>(api_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[actix_web::test]
    async fn test_list_leagues() {
        let leagues: Result<Vec<League>, Box<dyn Error>> = League::list().await;
        dbg!(&leagues);

        assert!(leagues.is_ok());
    }
}
