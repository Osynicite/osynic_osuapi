use crate::error::Result;
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

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestUsers {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IUsers for ReqwestUsers {
    async fn get_own_data(&self, mode: Option<Mode>, key: Option<String>) -> Result<User> {
        println!("ReqwestUsers get_own_data");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        // let params = [("mode", mode)];
        let params = [("key", key.map(|x| x.to_string()))];

        let mut url = "https://osu.ppy.sh/api/v2/me/".to_string();

        if mode.is_some() {
            url = format!(
                "https://osu.ppy.sh/api/v2/me/{}",
                mode.map(|x| x.to_ruleset()).unwrap_or_default()
            );
        }

        let response = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        // let response_text = response.text().await?;
        // println!("{:?}", response_text);
        // let user_response= User::default();

        // let text = response.text().await?;
        // println!("Response text: {:?}", text);
        // let json: User = serde_json::from_str(&text)?;
        // println!("Parsed JSON: {:?}", json);

        // // 获取响应体的文本内容
        // let text = response.text().await?;
        // println!("Response text: {}", text);

        // // 解析 JSON 数据
        // let json_value: serde_json::Value = serde_json::from_str(&text)?;
        // println!("Parsed JSON: {:?}", json_value);

        // // 将 JSON 数据解析为 User 结构体
        // let user: User = serde_json::from_str(&text)?;
        // println!("Parsed User: {:?}", user);

        // let json = serde_json::from_str::<User>(&response.text().await?)?;
        // println!("{:?}", json);

        // 解析响应

        let user_response: User = response.json().await?;

        Ok(user_response)
    }

    async fn get_user_kudosu(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<KudosuHisotry>> {
        println!("ReqwestUsers get_user_kudosu");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [
            ("limit", limit.map(|x| x.to_string())),
            ("offset", offset.map(|x| x.to_string())),
        ];

        let response = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/users/{}/kudosu", id))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

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
        println!("ReqwestUsers get_user_scores");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [
            ("legacy_only", legacy_only.map(|x| x.to_string())),
            ("limit", limit.map(|x| x.to_string())),
            ("offset", offset.map(|x| x.to_string())),
        ];

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/users/{}/scores/{}",
                id,
                score_type.to_param()
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

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
        println!("ReqwestUsers get_user_beatmaps");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [
            ("limit", limit.map(|x| x.to_string())),
            ("offset", offset.map(|x| x.to_string())),
        ];

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/users/{}/beatmapsets/{}",
                id, beatmap_type
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        let beatmapsets: Vec<Beatmapset> = response.json().await?;

        Ok(beatmapsets)
    }

    async fn get_user_beatmaps_most_played(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<BeatmapPlaycount>> {
        println!("ReqwestUsers get_user_beatmaps_most_played");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [
            ("limit", limit.map(|x| x.to_string())),
            ("offset", offset.map(|x| x.to_string())),
        ];

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/users/{}/beatmapsets/most_played",
                id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        let beatmap_playcount: Vec<BeatmapPlaycount> = response.json().await?;

        Ok(beatmap_playcount)
    }

    async fn get_user_recent_activity(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Event>> {
        println!("ReqwestUsers get_user_recent_activity");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [
            ("limit", limit.map(|x| x.to_string())),
            ("offset", offset.map(|x| x.to_string())),
        ];

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/users/{}/recent_activity",
                id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        let recent_activity: Vec<Event> = response.json().await?;

        Ok(recent_activity)
    }

    async fn get_user_by_username(
        &self,
        username: &str,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> Result<User> {
        println!("ReqwestUsers get_user_by_username");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [("key", key.map(|x| x.to_string()))];

        let url: String;

        if mode.is_none() {
            url = format!("https://osu.ppy.sh/api/v2/users/{}", username);
        } else {
            url = format!(
                "https://osu.ppy.sh/api/v2/users/{}/{}",
                username,
                mode.map(|x| x.to_ruleset()).unwrap_or_default()
            );
        }

        let response = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        let user_response: User = response.json().await?;

        Ok(user_response)
    }

    async fn get_user(&self, id: u32, mode: Option<Mode>, key: Option<String>) -> Result<User> {
        println!("ReqwestUsers get_user");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [("key", key.map(|x| x.to_string()))];

        // 如果mode是None，则是url1，否则是url2

        let url: String;

        if mode.is_none() {
            url = format!("https://osu.ppy.sh/api/v2/users/{}", id);
        } else {
            url = format!(
                "https://osu.ppy.sh/api/v2/users/{}/{}",
                id,
                mode.map(|x| x.to_ruleset()).unwrap_or_default()
            );
        }

        let response = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        // 获取响应体的文本内容
        // let text = response.text().await?;
        // println!("Response text: {}", text);

        // // 解析 JSON 数据
        // let json_value: serde_json::Value = serde_json::from_str(&text)?;
        // println!("Parsed JSON: {:?}", json_value);

        // // 将 JSON 数据解析为 User 结构体
        // let user: User = serde_json::from_str(&text)?;
        // println!("Parsed User: {:?}", user);

        // let user_response = User::default();

        let user_response: User = response.json().await?;

        Ok(user_response)
    }

    async fn get_users(
        &self,
        ids: Vec<u32>,
        include_variant_statistics: Option<bool>,
    ) -> Result<Users> {
        println!("ReqwestUsers get_users");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        // let params = [("ids", ids), ("include_variant_statistics", include_variant_statistics)];

        let ids_params = ids
            .iter()
            .take(50)
            .enumerate()
            .map(|(_, id)| ("ids[]".to_string(), id.to_string()))
            .collect::<Vec<(String, String)>>();
        let ivs_params = [("include_variant_statistics", include_variant_statistics)];

        let response = self
            .client
            .get("https://osu.ppy.sh/api/v2/users")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&ids_params)
            .query(&ivs_params)
            .send()
            .await?;

        println!("{:?}", response);

        let users_response: Users = response.json().await?;

        Ok(users_response)
    }
}
