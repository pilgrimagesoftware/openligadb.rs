use crate::constants::API_BASE_URL;
use crate::models::{
    goal::Goal, group::Group, location::Location, result::MatchResult, team::Team,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

use super::team;

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

impl Match {
    async fn get(
        id: i32
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}",
            API_BASE_URL, id
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn by_teams(
        team1_id: i32,
        team2_id: i32
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}",
            API_BASE_URL, team1_id, team2_id
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn by_league(
        league: &str,
        season: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}",
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

    async fn by_league_group(
        league: &str,
        season: i32,
        group_order_id: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}/{}",
            API_BASE_URL, league, season, group_order_id
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Vec<Self>>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn next_match_by_league_team(
        league: &str,
        team_id: i32
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getnextmatchbyleagueteam/{}/{}",
            API_BASE_URL, league, team_id
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn last_match_by_league_team(
        league: &str,
        team_id: i32
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyleagueteam/{}/{}",
            API_BASE_URL, league, team_id
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn next_match_by_league(
        league: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getnextmatchbyleagueshortcut/{}",
            API_BASE_URL, league
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn last_match_by_league(
        league: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyleagueshortcut/{}",
            API_BASE_URL, league
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Self>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn by_league_team(
        league: &str,
        season: i32,
        team_filter: &str
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}/{}",
            API_BASE_URL, league, season, team_filter
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Vec<Self>>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn by_team(
        team_filter: &str,
        week_count_past: i32,
        week_count_future: i32,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyteam/{}/{}/{}",
            API_BASE_URL, team_filter, week_count_past, week_count_future
        ))?;

        let response = reqwest::get(api_url.as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<Vec<Self>>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    async fn by_team_id(
        team_id: i32,
        week_count_past: i32,
        week_count_future: i32,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyteamid/{}/{}/{}",
            API_BASE_URL, team_id, week_count_past, week_count_future
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

    #[actix_web::test]
    async fn test_get() {
        let match_id = 1;
        let r#match: Result<Match, Box<dyn Error>> = Match::get(match_id).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[actix_web::test]
    async fn test_by_teams() {
        let team1 = 1;
        let team2 = 2;
        let r#match: Result<Match, Box<dyn Error>> = Match::by_teams(team1, team2).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[actix_web::test]
    async fn test_by_league() {
        let league = "bl1";
        let season = 2024;
        let matches: Result<Vec<Match>, Box<dyn Error>> = Match::by_league(league, season).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[actix_web::test]
    async fn test_by_league_group() {
        let league = "bl1";
        let season = 2024;
        let group_id = 1;
        let matches: Result<Vec<Match>, Box<dyn Error>> = Match::by_league_group(league, season, group_id).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[actix_web::test]
    async fn test_next_match_by_league_team() {
        let league = "bl1";
        let team_id = 1;
        let r#match: Result<Match, Box<dyn Error>> = Match::next_match_by_league_team(league, team_id).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[actix_web::test]
    async fn test_last_match_by_league_team() {
        let league = "bl1";
        let team_id = 1;
        let r#match: Result<Match, Box<dyn Error>> = Match::last_match_by_league_team(league, team_id).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[actix_web::test]
    async fn test_next_match_by_league() {
        let league = "bl1";
        let r#match: Result<Match, Box<dyn Error>> = Match::next_match_by_league(league).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[actix_web::test]
    async fn test_last_match_by_league() {
        let league = "bl1";
        let r#match: Result<Match, Box<dyn Error>> = Match::last_match_by_league(league).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[actix_web::test]
    async fn test_by_league_team() {
        let league = "bl1";
        let season = 2024;
        let team = "TODO";
        let matches: Result<Vec<Match>, Box<dyn Error>> = Match::by_league_team(league, season, team).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[actix_web::test]
    async fn test_by_team() {
        let team_filter = "TODO";
        let week_count_past = 1;
        let week_count_future = 1;
        let matches: Result<Vec<Match>, Box<dyn Error>> = Match::by_team(team_filter, week_count_past, week_count_future).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[actix_web::test]
    async fn test_by_team_id() {
        let team_id = 1;
        let week_count_past = 1;
        let week_count_future = 1;
        let matches: Result<Vec<Match>, Box<dyn Error>> = Match::by_team_id(team_id, week_count_past, week_count_future).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }
}
