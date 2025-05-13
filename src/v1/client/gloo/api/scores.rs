use crate::error::Result;
use crate::v1::client::gloo::check::check_res;
use crate::v1::interface::scores::IScores;
use crate::v1::model::scores::{GetScoresParams, Score};
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooScores {
    pub api_key: Arc<Mutex<String>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IScores for GlooScores {
    async fn get_scores(&self, params: GetScoresParams) -> Result<Vec<Score>> {
        console::log_1(&JsValue::from_str("GlooScore get_Scores"));

        let key = {
            let key = self.api_key.lock().unwrap();
            key.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let params = params.api_key(key).build_params();

        let url = format!(
            "{}https://osu.ppy.sh/api/get_Scores?{}",
            proxy_url,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = check_res(res)?;

        let scores: Vec<Score> = response.json().await?;

        Ok(scores)
    }
}
