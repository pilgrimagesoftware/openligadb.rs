#![doc = r"League table object and methods"]
use crate::constants::API_BASE_URL;
use crate::util;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

/// A data structure representing an entry in a league's table.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableTeam {
    /// The identifier of this table entry
    #[serde(rename(deserialize = "teamInfoId"))]
    pub id: i32,
    /// The name of the team; see [Team#name](crate::models::team::Team)
    #[serde(rename(deserialize = "teamName"))]
    pub name: Option<String>,
    /// The short name of the team; see [Team#short_name](crate::models::team::Team)
    #[serde(rename(deserialize = "shortName"))]
    pub short_name: Option<String>,
    /// A URL to the team's icon or logo
    #[serde(rename(deserialize = "teamIconUrl"))]
    pub icon_url: Option<String>,
    /// The number of points for the team
    #[serde(rename(deserialize = "points"))]
    pub points: i32,
    /// The total of goals scored against the team
    #[serde(rename(deserialize = "opponentGoals"))]
    pub opponent_goals: i32,
    /// The total of goals scored by the team
    #[serde(rename(deserialize = "goals"))]
    pub goals: i32,
    /// The number of matches played by the team
    #[serde(rename(deserialize = "matches"))]
    pub matches: i32,
    /// The number of matches won by the team
    #[serde(rename(deserialize = "won"))]
    pub wins: i32,
    /// The number of matches lost by the team
    #[serde(rename(deserialize = "lost"))]
    pub losses: i32,
    /// The number of matches drawn by the team
    #[serde(rename(deserialize = "draw"))]
    pub draws: i32,
    /// The team's goal differential
    #[serde(rename(deserialize = "goalDiff"))]
    pub goal_difference: i32,
}

impl TableTeam {
    /// Get the Bundesliga table
    ///
    /// Fetches the table for a Bundesliga tier and season
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    pub async fn get_bl_table(league: &str, season: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getbltable/{}/{}",
            API_BASE_URL, league, season
        ))?;

        util::list::<Self>(api_url).await
    }

    /// Get a league table
    ///
    /// Fetches the table for the specified league and season.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    pub async fn get_group_table(league: &str, season: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getgrouptable/{}/{}",
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
    async fn test_bl_table() {
        let season = 2024;
        let table: Result<Vec<TableTeam>, Box<dyn Error>> =
            TableTeam::get_bl_table(BUNDESLIGA, season).await;
        dbg!(&table);

        assert!(table.is_ok());
    }

    // TODO: enable when we find valid test data; nothing works so far
    // #[actix_web::test]
    // async fn test_group_table() {
    //     let season = 2024;
    //     let table: Result<Vec<TableTeam>, Box<dyn Error>> = TableTeam::get_group_table(BUNDESLIGA, season).await;
    //     dbg!(&table);

    //     assert!(table.is_ok());
    // }
}
