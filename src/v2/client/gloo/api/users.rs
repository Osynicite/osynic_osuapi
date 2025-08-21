use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::users::IUsers;
use crate::v2::model::beatmap::structs::beatmap_playcount::BeatmapPlaycount;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::event::structs::event::Event;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::enums::score_type::ScoreType;
use crate::v2::model::score::structs::score::Score;
use crate::v2::model::user::structs::kudosu_history::KudosuHisotry;
use crate::v2::model::user::structs::user::User;
use crate::v2::model::user::structs::users::Users;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooUsers {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IUsers for GlooUsers {
    async fn get_own_data(&self, mode: Option<Mode>, key: Option<String>) -> Result<User> {
        console::log_1(&JsValue::from_str("GlooUsers get_own_data"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(key) = key {
            query_params.push(("key", key));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/me")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/me?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let user: User = response.json().await?;
        Ok(user)
    }

    async fn get_user_kudosu(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<KudosuHisotry>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_kudosu"));

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
        if let Some(offset) = offset {
            query_params.push(("offset", offset));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/kudosu")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/kudosu?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let kudosu_history: Vec<KudosuHisotry> = response.json().await?;
        Ok(kudosu_history)
    }

    async fn get_user_scores(
        &self,
        id: u32,
        score_type: ScoreType,
        legacy_only: Option<u32>,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Score>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_scores"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(legacy) = legacy_only {
            query_params.push(("legacy_only", legacy.to_string()));
        }
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = offset {
            query_params.push(("offset", offset));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/scores/{score_type}")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/scores/{score_type}?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let scores: Vec<Score> = response.json().await?;
        Ok(scores)
    }

    async fn get_user_beatmaps(
        &self,
        id: u32,
        beatmap_type: String,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Beatmapset>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_beatmaps"));

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
        if let Some(offset) = offset {
            query_params.push(("offset", offset));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/beatmapsets/{beatmap_type}")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/beatmapsets/{beatmap_type}?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmapsets: Vec<Beatmapset> = response.json().await?;
        Ok(beatmapsets)
    }

    async fn get_user_beatmaps_most_played(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<BeatmapPlaycount>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_beatmaps_most_played"));

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
        if let Some(offset) = offset {
            query_params.push(("offset", offset));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/beatmapsets/most_played")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/beatmapsets/most_played?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap_playcounts: Vec<BeatmapPlaycount> = response.json().await?;
        Ok(beatmap_playcounts)
    }

    async fn get_user_recent_activity(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Event>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_recent_activity"));

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
        if let Some(offset) = offset {
            query_params.push(("offset", offset));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/recent_activity")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}/recent_activity?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let events: Vec<Event> = response.json().await?;
        Ok(events)
    }

    async fn get_user(
        &self,
        id: u32,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> Result<User> {
        console::log_1(&JsValue::from_str("GlooUsers get_user"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(key) = key {
            query_params.push(("key", key));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{id}?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let user: User = response.json().await?;
        Ok(user)
    }

    async fn get_user_by_username(
        &self,
        username: &str,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> Result<User> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_by_username"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(key) = key {
            query_params.push(("key", key));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{username}")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/users/{username}?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let user: User = response.json().await?;
        Ok(user)
    }

    async fn get_users(
        &self,
        ids: Vec<u32>,
        include_variant_statistics: Option<bool>,
    ) -> Result<Users> {
        console::log_1(&JsValue::from_str("GlooUsers get_users"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let ids_string = ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let mut query_params = vec![("ids[]", ids_string)];
        if let Some(include_variant) = include_variant_statistics {
            query_params.push(("include_variant_statistics", include_variant.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/users?{query_string}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let users: Users = response.json().await?;
        Ok(users)
    }
}
