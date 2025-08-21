use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::scores::IScores;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::structs::score::Score;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooScores {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IScores for GlooScores {
    async fn get_score(&self, mode: String, score_id: u64) -> Result<Score> {
        console::log_1(&JsValue::from_str("GlooScores get_score"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/scores/{}/{}",
            proxy_url, mode, score_id
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let score: Score = response.json().await?;

        Ok(score)
    }
}
