use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::beatmapsets::IBeatmapsets;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::search::dtos::params::BeatmapsetsSearchParams;
use crate::v2::model::search::dtos::response::BeatmapsetsSearchResponse;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooBeatmapsets {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IBeatmapsets for GlooBeatmapsets {
    async fn search(
        &self,
        params: BeatmapsetsSearchParams,
    ) -> Result<BeatmapsetsSearchResponse> {
        console::log_1(&JsValue::from_str("GlooBeatmapsets search"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        // Convert search params to query string
        let query_string = serde_urlencoded::to_string(&params)?;
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmapsets/search?{query_string}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let search_response: BeatmapsetsSearchResponse = response.json().await?;
        Ok(search_response)
    }

    async fn get_beatmapset(&self, beatmapset_id: u32) -> Result<Beatmapset> {
        console::log_1(&JsValue::from_str("GlooBeatmapsets get_beatmapset"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmapsets/{beatmapset_id}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmapset: Beatmapset = response.json().await?;
        Ok(beatmapset)
    }

    async fn download(&self, beatmapset_id: u32) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooBeatmapsets download"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmapsets/{beatmapset_id}/download");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        check_res(res)?;
        Ok(())
    }
}
