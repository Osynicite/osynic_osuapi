use crate::error::Result;
use crate::v1::client::request::check::check_res;
use crate::v1::interface::user::IUser;
use crate::v1::model::best::{BestScore, GetUserBestParams};
use crate::v1::model::recent::{GetUserRecentParams, RecentPlay};
use crate::v1::model::user::{GetUserParams, User};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestUser {
    pub client: reqwest::Client,
    pub api_key: Arc<RwLock<String>>,
}

impl IUser for ReqwestUser {
    async fn get_user(&self, params: GetUserParams) -> Result<Vec<User>> {
        println!("ReqwestUser get_user");

        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_user")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let user: Vec<User> = response.json().await?;

        // let text = response.text().await?;
        // println!("Text: {:?}", text);
        // let user: User = serde_json::from_str(&text)?;

        Ok(user)
    }

    async fn get_user_best(&self, params: GetUserBestParams) -> Result<Vec<BestScore>> {
        println!("ReqwestUserBest get_user_best");

        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_user_best")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let bests: Vec<BestScore> = response.json().await?;

        Ok(bests)
    }

    async fn get_user_recent(&self, params: GetUserRecentParams) -> Result<Vec<RecentPlay>> {
        println!("ReqwestUserRecent get_user_recent");

        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_user_recent")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let recents: Vec<RecentPlay> = response.json().await?;

        Ok(recents)
    }
}
