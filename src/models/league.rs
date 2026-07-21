#![doc = r"The League object and methods"]
#[cfg(feature = "http-client")]
use crate::constants::API_BASE_URL;
#[cfg(feature = "http-client")]
use crate::error::OpenLigaError;
use crate::models::sport::Sport;
#[cfg(feature = "http-client")]
use crate::util;
use serde::{Deserialize, Serialize};
#[cfg(feature = "http-client")]
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

#[cfg(feature = "http-client")]
impl League {
    /// List the leagues
    ///
    /// Fetches a list of leagues.
    pub async fn list() -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!("{}/getavailableleagues", API_BASE_URL))?;

        util::list::<Self>(api_url).await
    }
}

#[cfg(all(test, feature = "http-client"))]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_list_leagues() {
        let leagues: Result<Vec<League>, OpenLigaError> = League::list().await;
        dbg!(&leagues);

        assert!(leagues.is_ok());
    }
}
