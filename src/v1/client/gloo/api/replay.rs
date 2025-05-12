use crate::error::Result;
use crate::v1::client::gloo::check::check_res;
use crate::v1::interface::replay::IReplay;
use crate::v1::model::replay::{GetReplayParams, Replay};
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooReplay {
    pub api_key: Arc<Mutex<String>>,
}

impl IReplay for GlooReplay {
    async fn get_replay(&self, params: GetReplayParams) -> Result<Replay> {
        console::log_1(&JsValue::from_str("GlooReplay get_match"));

        let key = {
            let key = self.api_key.lock().unwrap();
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let url = format!(
            "https://osu.ppy.sh/api/get_match?{}",
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = check_res(res)?;

        let replay: Replay = response.json().await?;

        Ok(replay)
    }
}
