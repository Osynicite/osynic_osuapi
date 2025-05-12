use crate::error::Result;
use crate::v1::client::gloo::check::check_res;
use crate::v1::interface::multiplayer::IMultiplayer;
use crate::v1::model::multiplayer::{GetMatchParams, MultiplayerResponse};
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooMultiplayer {
    pub api_key: Arc<Mutex<String>>,
}

impl IMultiplayer for GlooMultiplayer {
    async fn get_match(&self, params: GetMatchParams) -> Result<MultiplayerResponse> {
        console::log_1(&JsValue::from_str("GlooMultiplayer get_match"));

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

        let multiplayer: MultiplayerResponse = response.json().await?;

        Ok(multiplayer)
    }
}
