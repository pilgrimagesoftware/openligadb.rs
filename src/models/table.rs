use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableTeam {
    #[serde(rename(deserialize = "teamInfoId"))]
    pub id: i32,
    #[serde(rename(deserialize = "teamName"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "shortName"))]
    pub short_name: Option<String>,
    #[serde(rename(deserialize = "teamIconUrl"))]
    pub icon_url: Option<String>,
    #[serde(rename(deserialize = "points"))]
    pub points: i32,
    #[serde(rename(deserialize = "opponentGoals"))]
    pub opponent_goals: i32,
    #[serde(rename(deserialize = "goals"))]
    pub goals: i32,
    #[serde(rename(deserialize = "matches"))]
    pub matches: i32,
    #[serde(rename(deserialize = "won"))]
    pub wins: i32,
    #[serde(rename(deserialize = "lost"))]
    pub losses: i32,
    #[serde(rename(deserialize = "draw"))]
    pub draws: i32,
    #[serde(rename(deserialize = "goalDiff"))]
    pub goal_difference: i32,
}

impl TableTeam {
    async fn get_bl_table(
        league: &str,
        season: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getbltable/{}/{}",
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

      async fn get_group_table(
        league: &str,
        season: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getgrouptable/{}/{}",
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

    const BUNDESLIGA: &str = "bl1";

    #[actix_web::test]
    async fn test_bl_table() {
        let league = BUNDESLIGA;
        let season = 2024;
        let table: Result<Vec<TableTeam>, Box<dyn Error>> = TableTeam::get_bl_table(league, season).await;
        dbg!(&table);

        assert!(table.is_ok());
    }

    #[actix_web::test]
    async fn test_group_table() {
        let league = BUNDESLIGA;
        let season = 2024;
        let table: Result<Vec<TableTeam>, Box<dyn Error>> = TableTeam::get_group_table(league, season).await;
        dbg!(&table);

        assert!(table.is_ok());
    }
}
