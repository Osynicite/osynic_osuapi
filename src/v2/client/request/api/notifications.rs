use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::notifications::INotifications;
use crate::v2::model::notification::dtos::request::MarkNotificationsRequest;
use crate::v2::model::notification::dtos::response::GetNotificationsResponse;
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestNotifications {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl INotifications for ReqwestNotifications {
    async fn get_notifications(
        &self,
        max_id: Option<String>,
    ) -> Result<GetNotificationsResponse> {
        println!("ReqwestNotifications get_notifications");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/notifications")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("max_id", max_id.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;
        
        let notifications: GetNotificationsResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let notifications: GetNotificationsResponse = serde_json::from_str(&text)?;

        Ok(notifications)
    }

    async fn mark_notifications_as_read(
            &self,
            params: Option<MarkNotificationsRequest>,
        ) -> Result<()> {
        println!("ReqwestNotifications mark_notifications_as_read");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .post("https://osu.ppy.sh/api/v2/notifications/mark-read")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        if response.status() == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            // 如果状态码不是 204，返回错误
            Err("Failed to mark notifications as read".to_string().into())
        }
    }
}

