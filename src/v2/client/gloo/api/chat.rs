use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::chat::IChat;
use crate::v2::model::chat::dtos::request::CreateChannelParams;
use crate::v2::model::chat::dtos::response::{
    CreateNewPMResponse, GetChannelResponse, GetUpdatesResponse,
};
use crate::v2::model::chat::structs::channel::ChatChannel;
use crate::v2::model::chat::structs::message::ChatMessage;
use crate::v2::model::chat::structs::silences::Silences;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooChat {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IChat for GlooChat {
    async fn chat_keepalive(
        &self,
        history_since: Option<u64>,
        since: Option<u64>,
    ) -> Result<Silences> {
        console::log_1(&JsValue::from_str("GlooChat chat_keepalive"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(history_since) = history_since {
            query_params.push(("history_since", history_since.to_string()));
        }
        if let Some(since) = since {
            query_params.push(("since", since.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/ack")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/ack?{query_string}")
        };

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let silences: Silences = response.json().await?;
        Ok(silences)
    }

    async fn create_new_pm(
        &self,
        target_id: u64,
        message: String,
        is_action: bool,
        uuid: Option<String>,
    ) -> Result<CreateNewPMResponse> {
        console::log_1(&JsValue::from_str("GlooChat create_new_pm"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut form_data = vec![
            format!("target_id={}", target_id),
            format!("message={}", message),
            format!("is_action={}", is_action),
        ];

        if let Some(uuid) = uuid {
            form_data.push(format!("uuid={uuid}"));
        }

        let body = form_data.join("&");
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/new");

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let create_pm_response: CreateNewPMResponse = response.json().await?;
        Ok(create_pm_response)
    }

    async fn get_updates(
        &self,
        history_since: Option<u64>,
        includes: Option<Vec<String>>,
        since: Option<u64>,
    ) -> Result<GetUpdatesResponse> {
        console::log_1(&JsValue::from_str("GlooChat get_updates"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(history_since) = history_since {
            query_params.push(("history_since", history_since.to_string()));
        }
        if let Some(includes) = includes {
            for include in includes {
                query_params.push(("includes[]", include));
            }
        }
        if let Some(since) = since {
            query_params.push(("since", since.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/updates")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/updates?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let updates_response: GetUpdatesResponse = response.json().await?;
        Ok(updates_response)
    }

    async fn get_channel_messages(
        &self,
        channel: String,
        limit: Option<u64>,
        since: Option<u64>,
        until: Option<u64>,
    ) -> Result<Vec<ChatMessage>> {
        console::log_1(&JsValue::from_str("GlooChat get_channel_messages"));

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
        if let Some(since) = since {
            query_params.push(("since", since.to_string()));
        }
        if let Some(until) = until {
            query_params.push(("until", until.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}/messages")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}/messages?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let messages: Vec<ChatMessage> = response.json().await?;
        Ok(messages)
    }

    async fn send_message_to_channel(
        &self,
        channel: u64,
        message: String,
        is_action: bool,
    ) -> Result<ChatMessage> {
        console::log_1(&JsValue::from_str("GlooChat send_message_to_channel"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = [
            format!("message={message}"),
            format!("is_action={is_action}"),
        ];

        let body = form_data.join("&");
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}/messages");

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let chat_message: ChatMessage = response.json().await?;
        Ok(chat_message)
    }

    async fn join_channel(&self, channel: String, user: String) -> Result<ChatChannel> {
        console::log_1(&JsValue::from_str("GlooChat join_channel"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = [format!("user={user}")];
        let body = form_data.join("&");
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}/users/{user}");

        let res = Request::put(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let chat_channel: ChatChannel = response.json().await?;
        Ok(chat_channel)
    }

    async fn leave_channel(&self, channel: String, user: String) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooChat leave_channel"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}/users/{user}");

        let res = Request::delete(&url)
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        check_res(res)?;
        Ok(())
    }

    async fn mark_channel_as_read(
        &self,
        channel: String,
        message: String,
        channel_id: String,
        message_id: String,
    ) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooChat mark_channel_as_read"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = [
            format!("message={message}"),
            format!("channel_id={channel_id}"),
            format!("message_id={message_id}"),
        ];

        let body = form_data.join("&");
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}/mark-as-read");

        let res = Request::put(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        check_res(res)?;
        Ok(())
    }

    async fn get_channel_list(&self) -> Result<Vec<ChatChannel>> {
        console::log_1(&JsValue::from_str("GlooChat get_channel_list"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let channels: Vec<ChatChannel> = response.json().await?;
        Ok(channels)
    }

    async fn create_channel(&self, params: CreateChannelParams) -> Result<ChatChannel> {
        console::log_1(&JsValue::from_str("GlooChat create_channel"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let body = serde_json::to_string(&params)?;
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels");

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let chat_channel: ChatChannel = response.json().await?;
        Ok(chat_channel)
    }

    async fn get_channel(&self, channel: String) -> Result<GetChannelResponse> {
        console::log_1(&JsValue::from_str("GlooChat get_channel"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/chat/channels/{channel}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let channel_response: GetChannelResponse = response.json().await?;
        Ok(channel_response)
    }
}
