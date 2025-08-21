use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::notifications::INotifications;
use crate::v2::model::notification::dtos::request::MarkNotificationsRequest;
use crate::v2::model::notification::dtos::response::GetNotificationsResponse;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooNotifications {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl INotifications for GlooNotifications {
    async fn get_notifications(
        &self,
        max_id: Option<String>,
    ) -> Result<GetNotificationsResponse> {
        console::log_1(&JsValue::from_str("GlooNotifications get_notifications"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(max_id) = max_id {
            query_params.push(("max_id", max_id));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/notifications", proxy_url)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/notifications?{}", proxy_url, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let notifications_response: GetNotificationsResponse = response.json().await?;
        Ok(notifications_response)
    }

    async fn mark_notifications_as_read(
        &self,
        params: Option<MarkNotificationsRequest>,
    ) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooNotifications mark_notifications_as_read"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{}https://osu.ppy.sh/api/v2/notifications/mark-read", proxy_url);

        let res = if let Some(params) = params {
            let body = serde_json::to_string(&params)?;
            Request::post(&url)
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .header("Authorization", &format!("Bearer {}", access_token))
                .body(body)?
                .send()
                .await?
        } else {
            Request::post(&url)
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .header("Authorization", &format!("Bearer {}", access_token))
                .send()
                .await?
        };

        check_res(res)?;
        Ok(())
    }
}
