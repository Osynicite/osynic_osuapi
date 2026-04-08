use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::friends::IFriends;
use crate::v2::model::friend::{Friend, FriendXApiVersion};
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestFriends {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IFriends for ReqwestFriends {
    async fn get_friends(&self) -> Result<Vec<Friend>> {
        println!("ReqwestFriends get_friends");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/friends")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let friends: Vec<Friend> = response.json().await?;

        Ok(friends)
    }

    async fn get_friends_x_api_version(
        &self,
        x_api_version: Option<String>,
    ) -> Result<Vec<FriendXApiVersion>> {
        println!("ReqwestFriends get_friends_x_api_version");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let x_api_version = x_api_version.unwrap_or("20251105".to_string());

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/friends")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("x-api-version", &x_api_version)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        // println!("Response: {:?}", response.text().await?);
        // 太长了帮我保存到本地txt吧！
        // std::fs::write("response.txt", response.text().await?)?;
        let friends_x_api_version: Vec<FriendXApiVersion> = response.json().await?;
        // let friends_x_api_version: Vec<FriendXApiVersion> = Vec::new();
        Ok(friends_x_api_version)
    }
}
