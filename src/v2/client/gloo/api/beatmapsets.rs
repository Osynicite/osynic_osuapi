use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::beatmapsets::IBeatmapsets;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooBeatmapsets {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IBeatmapsets for GlooBeatmapsets {
    async fn lookup_beatmapset(&self, beatmap_id: u32) -> Result<Beatmapset> {
        console::log_1(&JsValue::from_str("GlooBeatmapsets lookup_beatmapset"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [("beatmap_id", beatmap_id.to_string())];

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/beatmapsets/lookup?{}",
            proxy_url,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmapset: Beatmapset = response.json().await?;

        Ok(beatmapset)
    }

    async fn get_beatmapset(&self, beatmapset_id: u32) -> Result<Beatmapset> {
        console::log_1(&JsValue::from_str("GlooBeatmapsets get_beatmapset"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/beatmapsets/{}",
            proxy_url, beatmapset_id
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmapset: Beatmapset = response.json().await?;

        Ok(beatmapset)
    }
}
