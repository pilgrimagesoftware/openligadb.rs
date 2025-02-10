use crate::models::sport::Sport;
use serde::{Deserialize, Serialize};

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
