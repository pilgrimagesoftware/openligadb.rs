use crate::models::sport::Sport;
use crate::{constants::API_BASE_URL};
use crate::util;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    #[serde(rename(deserialize = "leagueId"))]
    pub id: i32,
    #[serde(rename(deserialize = "leagueName"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "leagueShortcut"))]
    pub shortcut: Option<String>,
    #[serde(rename(deserialize = "leagueSeason"))]
    pub season: Option<String>,
    pub sport: Sport,
}

impl League {
    async fn list() -> Result<Vec<Self>, Box<dyn Error>> {
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
