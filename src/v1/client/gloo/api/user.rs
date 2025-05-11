use crate::error::Result;
use crate::v1::client::gloo::check::check_res;
use crate::v1::interface::user::IUser;
use crate::v1::model::user::{User, GetUserParams};
use crate::v1::model::best::{BestScore,GetUserBestParams};
use crate::v1::model::recent::{GetUserRecentParams, RecentPlay};
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooUser {
    pub api_key: Arc<Mutex<String>>,
}

impl IUser for GlooUser {
    async fn get_user(&self, params: GetUserParams) -> Result<Vec<User>> {
        console::log_1(&JsValue::from_str("GlooUser get_user"));

        let key = {
            let key = self.api_key.lock().unwrap();
            key.clone()
        };

        let params = params.api_key(key).build_params();
        
        let url = format!(
            "https://osu.ppy.sh/api/get_users?{}",
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = check_res(res)?;
        
        let users: Vec<User> = response.json().await?;
        
        Ok(users)
    }

    async fn get_user_best(&self, params: GetUserBestParams) -> Result<Vec<BestScore>> {
        console::log_1(&JsValue::from_str("GlooUser get_user_best"));

        let key = {
            let key = self.api_key.lock().unwrap();
            key.clone()
        };

        let params = params.api_key(key).build_params();
        
        let url = format!(
            "https://osu.ppy.sh/api/get_user_best?{}",
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = check_res(res)?;
        
        let bests: Vec<BestScore> = response.json().await?;
        
        Ok(bests)
    }

    async fn get_user_recent(&self, params: GetUserRecentParams) -> Result<Vec<RecentPlay>> {
        console::log_1(&JsValue::from_str("GlooUser get_user_recent"));

        let key = {
            let key = self.api_key.lock().unwrap();
            key.clone()
        };

        let params = params.api_key(key).build_params();
        
        let url = format!(
            "https://osu.ppy.sh/api/get_user_recent?{}",
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = check_res(res)?;
        
        let recents: Vec<RecentPlay> = response.json().await?;
        
        Ok(recents)
    }
}