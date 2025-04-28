use crate::error::Result;
use crate::v1::client::request::check::check_res;
use crate::v1::interface::replay::IReplay;
use crate::v1::model::replay::{GetReplayParams, Replay};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestReplay {
    pub client: reqwest::Client,
    pub api_key: Arc<RwLock<String>>,
}

impl IReplay for ReqwestReplay {
    async fn get_replay(&self, params: GetReplayParams) -> Result<Replay> {
        println!("ReqwestReplay get_Replays");

        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_replay")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let replay: Replay = response.json().await?;

        Ok(replay)
    }
}
