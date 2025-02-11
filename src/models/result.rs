use crate::constants::API_BASE_URL;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalResultInfo {
    #[serde(rename(deserialize = "id"))]
    pub id: i32,
    #[serde(rename(deserialize = "name"))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResult {
    #[serde(rename(deserialize = "resultID"))]
    pub id: i32,
    #[serde(rename(deserialize = "resultName"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "pointsTeam1"))]
    pub points_team1: Option<i32>,
    #[serde(rename(deserialize = "pointsTeam2"))]
    pub points_team2: Option<i32>,
    #[serde(rename(deserialize = "resultOrderID"))]
    pub order_id: i32,
    #[serde(rename(deserialize = "resultTypeID"))]
    pub type_id: i32,
    #[serde(rename(deserialize = "resultDescription"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    #[serde(rename(deserialize = "id"))]
    pub id: i32,
    #[serde(rename(deserialize = "name"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "description"))]
    pub description: Option<String>,
    #[serde(rename(deserialize = "orderId"))]
    pub order_id: Option<i32>,
    #[serde(rename(deserialize = "globalResultInfo"))]
    pub global_result_info: GlobalResultInfo,
}

impl ResultInfo {
    async fn list(
        league_id: i32
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let api_url = Url::parse(&format!(
            "{}/getresultinfos/{}",
            API_BASE_URL, league_id
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

    const BUNDESLIGA_ID: i32 = 3;

    #[actix_web::test]
    async fn test_get() {
        let league = BUNDESLIGA_ID;
        let results: Result<Vec<ResultInfo>, Box<dyn Error>> = ResultInfo::list(league).await;
        dbg!(&results);

        assert!(results.is_ok());
    }

}
