use crate::error::Result;
use crate::v1::client::request::check::check_res;
use crate::v1::interface::scores::IScores;
use crate::v1::model::scores::{GetScoresParams, Score};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestScores {
    pub client: reqwest::Client,
    pub api_key: Arc<RwLock<String>>,
}

impl IScores for ReqwestScores {
    async fn get_scores(&self, params: GetScoresParams) -> Result<Vec<Score>> {
        println!("ReqwestScore get_scores");

        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_scores")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let scores: Vec<Score> = response.json().await?;

        Ok(scores)
    }
}
