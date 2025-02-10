use crate::models::sport::Sport;
use crate::{client::List, constants::API_BASE_URL};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    #[serde(rename(deserialize = "leagueId"))]
    pub id: u64,
    #[serde(rename(deserialize = "leagueName"))]
    pub name: String,
    #[serde(rename(deserialize = "leagueShortcut"))]
    pub shortcut: String,
    #[serde(rename(deserialize = "leagueSeason"))]
    pub season: String,
    pub sport: Sport,
}

#[async_trait]
impl<M: for<'de> serde::Deserialize<'de> + 'static> List<M> for League {
    async fn list() -> Result<Vec<M>, Box<dyn Error>> {
        let api_url = Url::parse(&format!("{}/getavailableleagues", API_BASE_URL))?;

        crate::client::list::<M>(api_url).await
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
