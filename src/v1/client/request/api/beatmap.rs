use crate::error::Result;
use crate::v1::client::request::check::check_res;
use crate::v1::interface::beatmap::IBeatmap;
use crate::v1::model::beatmap::{Beatmap, GetBeatmapsParams};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestBeatmap {
    pub client: reqwest::Client,
    pub api_key: Arc<RwLock<String>>,
}

impl IBeatmap for ReqwestBeatmap {
    async fn get_beatmaps(&self, params: GetBeatmapsParams) -> Result<Vec<Beatmap>> {
        println!("ReqwestBeatmap get_beatmaps");

        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_beatmaps")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let beatmaps: Vec<Beatmap> = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let beatmaps: Vec<Beatmap> = serde_json::from_str(&text)?;

        Ok(beatmaps)
    }
}
