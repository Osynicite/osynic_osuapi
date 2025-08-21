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
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooUsers {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IUsers for GlooUsers {
    async fn get_own_data(&self, mode: Option<Mode>, key: Option<String>) -> Result<User> {
        console::log_1(&JsValue::from_str("GlooUsers get_own_data"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [("key", key.map(|x| x.to_string()))];

        let mut url = format!("{}https://osu.ppy.sh/api/v2/me/", proxy_url);

        if let Some(mode) = mode {
            url = format!(
                "{}https://osu.ppy.sh/api/v2/me/{}",
                proxy_url,
                mode.to_ruleset()
            );
        }

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;
        let user_response: User = response.json().await?;

        Ok(user_response)
    }

    async fn get_user_kudosu(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<KudosuHisotry>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_kudosu"));

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
        if let Some(offset) = offset {
            params.push(("offset", offset));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/users/{}/kudosu?{}",
            proxy_url,
            id,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let kudosu_response: Vec<KudosuHisotry> = response.json().await?;

        Ok(kudosu_response)
    }

    async fn get_user_scores(
        &self,
        id: u32,
        r#type: ScoreType,
        include_fails: Option<bool>,
        mode: Option<Mode>,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Score>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_scores"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(include_fails) = include_fails {
            params.push(("include_fails", include_fails.to_string()));
        }
        if let Some(mode) = mode {
            params.push(("mode", mode.to_ruleset()));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = offset {
            params.push(("offset", offset));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/users/{}/scores/{}?{}",
            proxy_url,
            id,
            r#type.to_string(),
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let scores_response: Vec<Score> = response.json().await?;

        Ok(scores_response)
    }

    async fn get_user_beatmaps(
        &self,
        id: u32,
        r#type: String,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Beatmapset>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_beatmaps"));

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
        if let Some(offset) = offset {
            params.push(("offset", offset));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/users/{}/beatmapsets/{}?{}",
            proxy_url,
            id,
            r#type,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmapsets_response: Vec<Beatmapset> = response.json().await?;

        Ok(beatmapsets_response)
    }

    async fn get_user_most_played(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<BeatmapPlaycount>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_most_played"));

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
        if let Some(offset) = offset {
            params.push(("offset", offset));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/users/{}/beatmapsets/most_played?{}",
            proxy_url,
            id,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap_playcount_response: Vec<BeatmapPlaycount> = response.json().await?;

        Ok(beatmap_playcount_response)
    }

    async fn get_user(&self, id: u32, mode: Option<Mode>, key: Option<String>) -> Result<User> {
        console::log_1(&JsValue::from_str("GlooUsers get_user"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [("key", key.map(|x| x.to_string()))];

        let mut url = format!("{}https://osu.ppy.sh/api/v2/users/{}", proxy_url, id);

        if let Some(mode) = mode {
            url = format!(
                "{}https://osu.ppy.sh/api/v2/users/{}/{}",
                proxy_url,
                id,
                mode.to_ruleset()
            );
        }

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;
        let user_response: User = response.json().await?;

        Ok(user_response)
    }

    async fn get_users(&self, ids: Vec<u32>) -> Result<Users> {
        console::log_1(&JsValue::from_str("GlooUsers get_users"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let ids_str = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let params = [("ids[]", ids_str)];

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/users?{}",
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
        let users_response: Users = response.json().await?;

        Ok(users_response)
    }

    async fn get_user_recent_activity(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Event>> {
        console::log_1(&JsValue::from_str("GlooUsers get_user_recent_activity"));

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
        if let Some(offset) = offset {
            params.push(("offset", offset));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/users/{}/recent_activity?{}",
            proxy_url,
            id,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let activity_response: Vec<Event> = response.json().await?;

        Ok(activity_response)
    }
}
