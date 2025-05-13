use crate::error::Result;
use crate::v1::client::gloo::check::check_res;
use crate::v1::interface::beatmap::IBeatmap;
use crate::v1::model::beatmap::{Beatmap, GetBeatmapsParams};
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooBeatmap {
    pub api_key: Arc<Mutex<String>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IBeatmap for GlooBeatmap {
    async fn get_beatmaps(&self, params: GetBeatmapsParams) -> Result<Vec<Beatmap>> {
        console::log_1(&JsValue::from_str("GlooBeatmap get_beatmaps"));

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
            "{}https://osu.ppy.sh/api/get_beatmaps?{}",
            proxy_url,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = check_res(res)?;

        let beatmaps: Vec<Beatmap> = response.json().await?;

        Ok(beatmaps)
    }
}
