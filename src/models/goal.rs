#![doc = r"Goal objects"]
use crate::constants::API_BASE_URL;
use crate::util;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

/// A data structure representing a goal
#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    /// The identifier of this goal data
    #[serde(rename(deserialize = "goalID"))]
    pub id: i32,
    /// The score of the first team when the goal was scored
    #[serde(rename(deserialize = "scoreTeam1"))]
    pub score_team1: Option<i32>,
    /// The score of the second team when the goal was scored
    #[serde(rename(deserialize = "scoreTeam2"))]
    pub score_team2: Option<i32>,
    /// The minute in the match when the goal was scored
    #[serde(rename(deserialize = "matchMinute"))]
    pub match_minute: Option<i32>,
    /// The identifier of the goal getter data
    #[serde(rename(deserialize = "goalGetterID"))]
    pub goal_getter_id: i32,
    /// The name of the player scoring the goal
    #[serde(rename(deserialize = "goalGetterName"))]
    pub goal_getter_name: Option<String>,
    /// Indicates whether the goal scored was the result of a penalty kick
    #[serde(rename(deserialize = "isPenalty"))]
    pub is_penalty: Option<bool>,
    /// Indicates of the goal was an own-goal
    #[serde(rename(deserialize = "isOwnGoal"))]
    pub is_own_goal: Option<bool>,
    /// Indicates if the goal was scored during overtime
    #[serde(rename(deserialize = "isOvertime"))]
    pub is_overtime: Option<bool>,
    /// A comment associated with the goal
    #[serde(rename(deserialize = "comment"))]
    pub comment: Option<String>,
}

/// A data structure representing a goal getter, i.e., the scorer of a goal.
#[derive(Debug, Serialize, Deserialize)]
pub struct GoalGetter {
    /// The identifier of this goal getter data
    #[serde(rename(deserialize = "goalGetterId"))]
    pub id: i32,
    /// The name of the player scoring the goal
    #[serde(rename(deserialize = "goalGetterName"))]
    pub name: Option<String>,
    /// The number of goals
    #[serde(rename(deserialize = "goalCount"))]
    pub goal_count: i32,
}

impl GoalGetter {
    /// List goal getters
    ///
    /// Retrieve a list of all goal getters for a particular league's season.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season value, usually the year the season begins
    pub async fn list(league: &str, season: i32) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getgoalgetters/{}/{}",
            API_BASE_URL, league, season
        ))?;

        util::list(api_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const BUNDESLIGA: &str = "bl1";

    #[actix_web::test]
    async fn test_list() {
        let league = BUNDESLIGA;
        let season = 2024;
        let teams: Result<Vec<GoalGetter>, Box<dyn Error>> = GoalGetter::list(league, season).await;
        dbg!(&teams);

        assert!(teams.is_ok());
    }
}
