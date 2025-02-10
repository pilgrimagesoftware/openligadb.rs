use crate::{client::List, constants::API_BASE_URL};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sport {
    #[serde(rename(deserialize = "sportId"))]
    pub id: u64,
    #[serde(rename(deserialize = "sportName"))]
    pub name: String,
}

#[async_trait]
impl<M: for<'de> serde::Deserialize<'de> + 'static> List<M> for Sport {
    async fn list() -> Result<Vec<M>, Box<dyn Error>> {
        let api_url = Url::parse(&format!("{}/getavailablesports", API_BASE_URL))?;

        crate::client::list::<M>(api_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[actix_web::test]
    async fn test_list_sports() {
        let sports: Result<Vec<Sport>, Box<dyn Error>> = Sport::list().await;
        dbg!(&sports);

        assert!(sports.is_ok());
    }
}
