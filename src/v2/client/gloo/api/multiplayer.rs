use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::multiplayer::IMultiplayer;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::room::structs::room::Room;
use crate::v2::model::score::structs::multiplayer::multiplayer_scores::MultiplayerScores;
use crate::v2::model::score::structs::score::Score;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooMultiplayer {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IMultiplayer for GlooMultiplayer {
    async fn get_user_high_score(
        &self,
        room: String,
        playlist: u64,
        user: u64,
    ) -> Result<Score> {
        console::log_1(&JsValue::from_str("GlooMultiplayer get_user_high_score"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores/users/{}",
            proxy_url, room, playlist, user
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

    async fn get_scores(
        &self,
        room: String,
        playlist: u64,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<MultiplayerScores> {
        console::log_1(&JsValue::from_str("GlooMultiplayer get_scores"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }
        if let Some(cursor) = cursor_string {
            query_params.push(("cursor_string", cursor));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!(
                "{}https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores",
                proxy_url, room, playlist
            )
        } else {
            format!(
                "{}https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores?{}",
                proxy_url, room, playlist, query_string
            )
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let multiplayer_scores: MultiplayerScores = response.json().await?;
        Ok(multiplayer_scores)
    }

    async fn get_score(
        &self,
        room: String,
        playlist: u64,
        score: u64,
    ) -> Result<Score> {
        console::log_1(&JsValue::from_str("GlooMultiplayer get_score"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores/{}",
            proxy_url, room, playlist, score
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

    async fn get_multiplayer_rooms(
        &self,
        limit: Option<u32>,
        mode: Option<String>,
        season_id: Option<u32>,
        sort: Option<String>,
        type_group: Option<String>,
    ) -> Result<Vec<Room>> {
        console::log_1(&JsValue::from_str("GlooMultiplayer get_multiplayer_rooms"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(mode) = mode {
            query_params.push(("mode", mode));
        }
        if let Some(season_id) = season_id {
            query_params.push(("season_id", season_id.to_string()));
        }
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }
        if let Some(type_group) = type_group {
            query_params.push(("type_group", type_group));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/rooms", proxy_url)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/rooms?{}", proxy_url, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let rooms: Vec<Room> = response.json().await?;
        Ok(rooms)
    }
}
