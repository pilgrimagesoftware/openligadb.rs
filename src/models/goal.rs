use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;
use crate::util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    #[serde(rename(deserialize = "goalID"))]
    pub id: i32,
    #[serde(rename(deserialize = "scoreTeam1"))]
    pub score_team1: Option<i32>,
    #[serde(rename(deserialize = "scoreTeam2"))]
    pub score_team2: Option<i32>,
    #[serde(rename(deserialize = "matchMinute"))]
    pub match_minute: Option<i32>,
    #[serde(rename(deserialize = "goalGetterID"))]
    pub goal_getter_id: i32,
    #[serde(rename(deserialize = "goalGetterName"))]
    pub goal_getter_name: Option<String>,
    #[serde(rename(deserialize = "isPenalty"))]
    pub is_penalty: Option<bool>,
    #[serde(rename(deserialize = "isOwnGoal"))]
    pub is_own_goal: Option<bool>,
    #[serde(rename(deserialize = "isOvertime"))]
    pub is_overtime: Option<bool>,
    #[serde(rename(deserialize = "comment"))]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalGetter {
    #[serde(rename(deserialize = "goalGetterId"))]
    pub id: i32,
    #[serde(rename(deserialize = "goalGetterName"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "goalCount"))]
    pub goal_count: i32,
}

impl GoalGetter {
    async fn list(
        league: &str,
        season: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
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
