use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::notifications::INotifications;
use crate::v2::model::notification::structs::notification::Notification;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooNotifications {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl INotifications for GlooNotifications {
    async fn get_notifications(
        &self,
        max_id: Option<u32>,
        unread_only: Option<bool>,
    ) -> Result<Vec<Notification>> {
        console::log_1(&JsValue::from_str("GlooNotifications get_notifications"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(max_id) = max_id {
            params.push(("max_id", max_id.to_string()));
        }
        if let Some(unread_only) = unread_only {
            params.push(("unread_only", unread_only.to_string()));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/notifications?{}",
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
        let notifications: Vec<Notification> = response.json().await?;

        Ok(notifications)
    }

    async fn mark_notifications_as_read(&self, identities: Vec<serde_json::Value>) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooNotifications mark_notifications_as_read"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let body = serde_json::json!({
            "identities": identities
        });

        let url = format!("{}https://osu.ppy.sh/api/v2/notifications/mark-read", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .body(body.to_string())
            .send()
            .await?;

        let _response = check_res(res)?;

        Ok(())
    }
}
