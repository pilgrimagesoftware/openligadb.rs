#![doc = r"Match result objects and methods"]
use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;
use crate::util;

/// A data structure representing a global result.
/// This is referenced from a [ResultInfo] object.
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalResultInfo {
    /// The identifier of the global result data
    #[serde(rename(deserialize = "id"))]
    pub id: i32,
    /// The name of the result
    #[serde(rename(deserialize = "name"))]
    pub name: Option<String>,
}

/// A data structure representing a match result
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResult {
    /// The identifier of the match result
    #[serde(rename(deserialize = "resultID"))]
    pub id: i32,
    /// The name of the result
    #[serde(rename(deserialize = "resultName"))]
    pub name: Option<String>,
    /// The score of the first team in the match
    #[serde(rename(deserialize = "pointsTeam1"))]
    pub points_team1: Option<i32>,
    /// The score of the second team in the match
    #[serde(rename(deserialize = "pointsTeam2"))]
    pub points_team2: Option<i32>,
    /// The result order identifier
    #[serde(rename(deserialize = "resultOrderID"))]
    pub order_id: i32,
    /// The result type identifier
    #[serde(rename(deserialize = "resultTypeID"))]
    pub type_id: i32,
    /// The result description
    #[serde(rename(deserialize = "resultDescription"))]
    pub description: Option<String>,
}

/// A data structure representing a result
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    /// The identifier of this result data
    #[serde(rename(deserialize = "id"))]
    pub id: i32,
    /// The name of the result
    #[serde(rename(deserialize = "name"))]
    pub name: Option<String>,
    /// A description of the result
    #[serde(rename(deserialize = "description"))]
    pub description: Option<String>,
    /// The order identifier of the result
    #[serde(rename(deserialize = "orderId"))]
    pub order_id: Option<i32>,
    /// A [GlobalResultInfo] object
    #[serde(rename(deserialize = "globalResultInfo"))]
    pub global_result_info: Option<GlobalResultInfo>,
}

impl ResultInfo {
    /// Get league results
    ///
    /// Fetches a list of results for the specified league.
    ///
    /// * `league_id` - The identifier of the league; see [League#id](crate::models::league::League)
    pub async fn list(
        league_id: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getresultinfos/{}",
            API_BASE_URL, league_id
        ))?;

        util::list::<Self>(api_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const BUNDESLIGA_ID: i32 = 3;

    #[actix_web::test]
    async fn test_list() {
        let league = BUNDESLIGA_ID;
        let results: Result<Vec<ResultInfo>, Box<dyn Error>> = ResultInfo::list(league).await;
        dbg!(&results);

        assert!(results.is_ok());
    }

}
