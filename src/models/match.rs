#![doc = r"The Match object and methods"]
#[cfg(feature = "http-client")]
use crate::constants::API_BASE_URL;
#[cfg(feature = "http-client")]
use crate::error::OpenLigaError;
use crate::models::{
    goal::Goal, group::Group, location::Location, result::MatchResult, team::Team,
};
#[cfg(feature = "http-client")]
use crate::util;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "http-client")]
use url::Url;

/// A data structure representing a match
#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    /// The identifier of the match data
    #[serde(rename(deserialize = "matchID"))]
    pub id: i32,
    /// The time when the match took place, or will take place
    #[serde(rename(deserialize = "matchDateTime"))]
    pub when: Option<NaiveDateTime>,
    /// The time zone of the match
    #[serde(rename(deserialize = "timeZoneID"))]
    pub time_zone: Option<String>,
    /// The identifier of the league
    #[serde(rename(deserialize = "leagueId"))]
    pub league_id: i32,
    /// The name of the league
    #[serde(rename(deserialize = "leagueName"))]
    pub league_name: Option<String>,
    /// The season of this match
    #[serde(rename(deserialize = "leagueSeason"))]
    pub league_season: Option<i32>,
    /// The league shortcut; see [League#shortcut](crate::models::league::League)
    #[serde(rename(deserialize = "leagueShortcut"))]
    pub league_shortcut: Option<String>,
    /// The date and time in UTC for the match
    #[serde(rename(deserialize = "matchDateTimeUTC"))]
    pub when_utc: Option<DateTime<Utc>>,
    /// The group to which this match belongs
    #[serde(rename(deserialize = "group"))]
    pub group: Group,
    /// The first team's information
    #[serde(rename(deserialize = "team1"))]
    pub team1: Team,
    /// The second team's information
    #[serde(rename(deserialize = "team2"))]
    pub team2: Team,
    /// The timestamp when this data was last updated
    #[serde(rename(deserialize = "lastUpdateDateTime"))]
    pub last_update: Option<NaiveDateTime>,
    /// Indicates if the match is finished or in-progress
    #[serde(rename(deserialize = "matchIsFinished"))]
    pub is_finished: bool,
    /// The match results
    #[serde(rename(deserialize = "matchResults"))]
    pub results: Option<Vec<MatchResult>>,
    /// The number of goals scored in the match
    #[serde(rename(deserialize = "goals"))]
    pub goals: Option<Vec<Goal>>,
    /// The location where the match was played
    #[serde(rename(deserialize = "location"))]
    pub location: Option<Location>,
    /// The number of viewers of the match
    #[serde(rename(deserialize = "numberOfViewers"))]
    pub number_of_viewers: Option<i32>,
}

#[cfg(feature = "http-client")]
impl Match {
    /// Get a match
    ///
    /// Fetches the specified match; see [Match#id](Match)
    ///
    /// * `id` - The identifier of the match
    pub async fn get(id: i32) -> Result<Self, OpenLigaError> {
        let api_url = Url::parse(&format!("{}/getmatchdata/{}", API_BASE_URL, id))?;

        util::get::<Self>(api_url).await
    }

    /// Get matches played by two teams
    ///
    /// Fetches a list of matches played by the two specified teams.
    ///
    /// * `team1_id` - The identifier of the first team
    /// * `team2_id` - The identifier of the second team
    pub async fn by_teams(team1_id: i32, team2_id: i32) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}",
            API_BASE_URL, team1_id, team2_id
        ))?;

        util::list::<Self>(api_url).await
    }

    /// Get a league's season matches
    ///
    /// Fetches all the matches played in a league's specified season.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    pub async fn by_league(league: &str, season: i32) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}",
            API_BASE_URL, league, season
        ))?;

        util::list::<Self>(api_url).await
    }

    /// Get matches for a league group
    ///
    /// Fetches a list of matches for the specified league, season, and group order.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    /// * `group_order_id` - The identifier of the group order
    pub async fn by_league_group(
        league: &str,
        season: i32,
        group_order_id: i32,
    ) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}/{}",
            API_BASE_URL, league, season, group_order_id
        ))?;

        util::list::<Self>(api_url).await
    }

    /// Get a team's next match
    ///
    /// Fetches the next match for the specified team.
    ///
    /// * `league_id` - The numeric league ID; see [League#id](crate::models::league::League)
    /// * `team_id` - The identifier of the team; see [Team#id](crate::models::team::Team)
    pub async fn next_match_by_league_team(
        league_id: i32,
        team_id: i32,
    ) -> Result<Self, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getnextmatchbyleagueteam/{}/{}",
            API_BASE_URL, league_id, team_id
        ))?;

        util::get::<Self>(api_url).await
    }

    /// Get a team's last match
    ///
    /// Fetches the most recently played match for the specified team.
    ///
    /// * `league_id` - The numeric league ID; see [League#id](crate::models::league::League)
    /// * `team_id` - The identifier of the team; see [Team#id](crate::models::team::Team)
    pub async fn last_match_by_league_team(
        league_id: i32,
        team_id: i32,
    ) -> Result<Self, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyleagueteam/{}/{}",
            API_BASE_URL, league_id, team_id
        ))?;

        util::get::<Self>(api_url).await
    }

    /// Get the next league match
    ///
    /// Fetches the next match to be played in the specified league.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    pub async fn next_match_by_league(league: &str) -> Result<Self, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getnextmatchbyleagueshortcut/{}",
            API_BASE_URL, league
        ))?;

        util::get::<Self>(api_url).await
    }

    /// Get the last league match
    ///
    /// Fetches the most recently played match for the specified league.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    pub async fn last_match_by_league(league: &str) -> Result<Self, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getlastmatchbyleagueshortcut/{}",
            API_BASE_URL, league
        ))?;

        util::get::<Self>(api_url).await
    }

    /// Get teams for a league's season
    ///
    /// Fetches a list of teams that match the filter, within a specific league and season.
    ///
    /// * `league` - The league shortcut; see [League#shortcut](crate::models::league::League)
    /// * `season` - The season, usually the starting year
    /// * `team_filter` - A search string for the team
    pub async fn by_league_team(
        league: &str,
        season: i32,
        team_filter: &str,
    ) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getmatchdata/{}/{}/{}",
            API_BASE_URL, league, season, team_filter
        ))?;

        util::list::<Self>(api_url).await
    }

    /// Get a range of teams' matches
    ///
    /// Fetches all the matches for teams that match the filter within the specified time window, relative
    /// to the current date.
    ///
    /// * `team_filter` - A search string for the team
    /// * `week_count_past` - The number of weeks in the past
    /// * `week_count_future` - The number of weeks in the future
    pub async fn by_team_range(
        team_filter: &str,
        week_count_past: i32,
        week_count_future: i32,
    ) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getmatchesbyteam/{}/{}/{}",
            API_BASE_URL, team_filter, week_count_past, week_count_future
        ))?;

        util::list::<Self>(api_url).await
    }

    /// Get a range of team matches
    ///
    /// Fetches all the matches for a specific team within the specified time window, relative
    /// to the current date.
    ///
    /// * `team_id` - The identifier of the team
    /// * `week_count_past` - The number of weeks in the past
    /// * `week_count_future` - The number of weeks in the future
    pub async fn by_team_id_range(
        team_id: i32,
        week_count_past: i32,
        week_count_future: i32,
    ) -> Result<Vec<Self>, OpenLigaError> {
        let api_url = Url::parse(&format!(
            "{}/getmatchesbyteamid/{}/{}/{}",
            API_BASE_URL, team_id, week_count_past, week_count_future
        ))?;

        util::list::<Self>(api_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[cfg(feature = "http-client")]
    const LEIPZIG_TEAM_ID: i32 = 1635;
    #[cfg(feature = "http-client")]
    const KIEL_TEAM_ID: i32 = 104;
    #[cfg(feature = "http-client")]
    const LEIPZIG_STPAULI_MATCH_ID: i32 = 72395;
    #[cfg(feature = "http-client")]
    const BUNDESLIGA: &str = "bl1";
    #[cfg(feature = "http-client")]
    const BUNDESLIGA_ID: i32 = 4741;

    #[test]
    fn test_deserialize_match() {
        let f = File::open("data/match-72395.json").unwrap();
        let result: Result<Match, serde_json::Error> = serde_json::from_reader(f);
        dbg!(&result);

        assert!(result.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_get() {
        let match_id = LEIPZIG_STPAULI_MATCH_ID; // Leipzig - St. Pauli
        let r#match: Result<Match, OpenLigaError> = Match::get(match_id).await;
        dbg!(&r#match);

        assert!(&r#match.is_ok());
        assert_eq!(&r#match.as_ref().unwrap().id, &match_id);
        assert!(&r#match.as_ref().unwrap().team1.short_name.is_some());
        // assert_eq!(r#match.as_ref().unwrap().team1.short_name.as_ref().unwrap(), "Leipzig".to_string());
        assert!(&r#match.as_ref().unwrap().group.name.is_some());
        // assert_eq!(&r#match.as_ref().unwrap().group.name.unwrap(), "21. Spieltag");
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_by_teams() {
        let team1 = LEIPZIG_TEAM_ID; // Leipzig
        let team2 = KIEL_TEAM_ID; // Holstein Kiel
        let result: Result<Vec<Match>, OpenLigaError> = Match::by_teams(team1, team2).await;
        dbg!(&result);

        assert!(result.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_by_league() {
        let league = BUNDESLIGA;
        let season = 2024;
        let matches: Result<Vec<Match>, OpenLigaError> = Match::by_league(league, season).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_by_league_group() {
        let league = BUNDESLIGA;
        let season = 2024;
        let group_id = 1;
        let matches: Result<Vec<Match>, OpenLigaError> =
            Match::by_league_group(league, season, group_id).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    // TODO: fix to get current league
    // #[actix_web::test]
    // async fn test_next_match_by_league_team() {
    //     let league_id = BUNDESLIGA_ID;
    //     let team_id = LEIPZIG_TEAM_ID;
    //     let r#match: Result<Match, OpenLigaError> =
    //         Match::next_match_by_league_team(league_id, team_id).await;
    //     dbg!(&r#match);

    //     assert!(r#match.is_ok());
    // }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_last_match_by_league_team() {
        let league_id = BUNDESLIGA_ID;
        let team_id = LEIPZIG_TEAM_ID;
        let result: Result<Match, OpenLigaError> =
            Match::last_match_by_league_team(league_id, team_id).await;
        dbg!(&result);

        assert!(result.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_next_match_by_league() {
        let league = BUNDESLIGA;
        let result: Result<Match, OpenLigaError> = Match::next_match_by_league(league).await;
        dbg!(&result);

        assert!(result.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_last_match_by_league() {
        let league = BUNDESLIGA;
        let r#match: Result<Match, OpenLigaError> = Match::last_match_by_league(league).await;
        dbg!(&r#match);

        assert!(r#match.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_by_league_team() {
        let league = BUNDESLIGA;
        let season = 2024;
        let team = "Leipzig";
        let matches: Result<Vec<Match>, OpenLigaError> =
            Match::by_league_team(league, season, team).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_by_team_range() {
        let team_filter = "Leipzig";
        let week_count_past = 1;
        let week_count_future = 1;
        let matches: Result<Vec<Match>, OpenLigaError> =
            Match::by_team_range(team_filter, week_count_past, week_count_future).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }

    #[cfg(feature = "http-client")]
    #[actix_web::test]
    async fn test_by_team_id_range() {
        let team_id = LEIPZIG_TEAM_ID;
        let week_count_past = 1;
        let week_count_future = 1;
        let matches: Result<Vec<Match>, OpenLigaError> =
            Match::by_team_id_range(team_id, week_count_past, week_count_future).await;
        dbg!(&matches);

        assert!(matches.is_ok());
    }
}
