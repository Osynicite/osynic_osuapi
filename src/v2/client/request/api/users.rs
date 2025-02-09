use crate::error::Result;
use crate::v2::interface::users::IUsers;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::users::structs::user::User;
use crate::v2::model::mode::enums::mode::Mode;


use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReqwestUsers {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IUsers for ReqwestUsers {
    async fn get_own_data(
        &self,
        mode: Option<Mode>,
    ) -> Result<User> {
        println!("ReqwestUsers get_own_data");
        
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [("mode", mode)];

        let response = self
            .client
            .get("https://osu.ppy.sh/api/v2/me")
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

    async fn get_user(
        &self,
        id: &str,
        mode: Option<Mode>,
    ) -> Result<User> {
        println!("ReqwestUsers get_user");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let params = [("mode", mode)];

        let response = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/users/{}", id))
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
}
