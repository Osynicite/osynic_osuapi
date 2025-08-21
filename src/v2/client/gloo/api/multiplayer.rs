use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::multiplayer::IMultiplayer;
use crate::v2::model::multiplayer::structs::multiplayer_scores::MultiplayerScores;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooMultiplayer {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IMultiplayer for GlooMultiplayer {
    async fn get_multiplayer_scores(
        &self,
        room_id: u32,
        playlist_id: u32,
        limit: Option<i32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<MultiplayerScores> {
        console::log_1(&JsValue::from_str("GlooMultiplayer get_multiplayer_scores"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(sort) = sort {
            params.push(("sort", sort));
        }
        if let Some(cursor_string) = cursor_string {
            params.push(("cursor_string", cursor_string));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores?{}",
            proxy_url,
            room_id,
            playlist_id,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let scores: MultiplayerScores = response.json().await?;

        Ok(scores)
    }
}
