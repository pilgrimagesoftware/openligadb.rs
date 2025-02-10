use serde::{Deserialize, Serialize};
use crate::models::{group::Group, team::Team, result::MatchResult, goal::Goal, location::Location};

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename(deserialize = "matchID"))]
    pub id: u64,
    #[serde(rename(deserialize = "matchDateTime"))]
    pub when: Option<String>,
    #[serde(rename(deserialize = "timeZoneID"))]
    pub time_zone: Option<String>,
    #[serde(rename(deserialize = "leagueId"))]
    pub league_id: i32,
    #[serde(rename(deserialize = "leagueName"))]
    pub league_name: Option<String>,
    #[serde(rename(deserialize = "leagueSeason"))]
    pub league_season: Option<String>,
    #[serde(rename(deserialize = "leagueShortcut"))]
    pub league_shortcut: Option<String>,
    #[serde(rename(deserialize = "matchDateTimeUTC"))]
    pub when_utc: Option<String>,
    #[serde(rename(deserialize = "group"))]
    pub group: Group,
    #[serde(rename(deserialize = "team1"))]
    pub team1: Team,
    #[serde(rename(deserialize = "team2"))]
    pub team2: Team,
    #[serde(rename(deserialize = "lastUpdateDateTime"))]
    pub last_update: Option<String>,
    #[serde(rename(deserialize = "matchIsFinished"))]
    pub is_finished: bool,
    #[serde(rename(deserialize = "matchResults"))]
    pub results: Option<Vec<MatchResult>>,
    #[serde(rename(deserialize = "goals"))]
    pub goals: Option<Vec<Goal>>,
    #[serde(rename(deserialize = "location"))]
    pub location: Location,
    #[serde(rename(deserialize = "numberOfViewers"))]
    pub number_of_viewers: Option<i32>,
}
