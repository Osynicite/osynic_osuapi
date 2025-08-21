use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::matches::IMatches;
use crate::v2::model::r#match::structs::r#match::Match;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooMatches {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IMatches for GlooMatches {
    async fn get_match(&self, match_id: u32) -> Result<Match> {
        console::log_1(&JsValue::from_str("GlooMatches get_match"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/matches/{}",
            proxy_url, match_id
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let match_data: Match = response.json().await?;

        Ok(match_data)
    }
}
