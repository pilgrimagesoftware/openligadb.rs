use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct BundesligaTableTeam {
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
