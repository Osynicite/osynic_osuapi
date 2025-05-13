use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::chat::IChat;
use crate::v2::model::chat::dtos::request::CreateChannelParams;
use crate::v2::model::chat::dtos::response::{
    CreateNewPMResponse, GetChannelResponse, GetUpdatesResponse,
};
use crate::v2::model::chat::structs::channel::ChatChannel;
use crate::v2::model::chat::structs::message::ChatMessage;
use crate::v2::model::chat::structs::silences::Silences;
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestChat {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IChat for ReqwestChat {
    async fn chat_keepalive(
        &self,
        history_since: Option<u64>,
        since: Option<u64>,
    ) -> Result<Silences> {
        println!("ReqwestChat chat_keepalive");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .post("https://osu.ppy.sh/api/v2/chat/ack")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("history_since", history_since.map(|s| s.to_string())),
                ("since", since.map(|s| s.to_string())),
            ])
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
        println!("ReqwestChat create_new_pm");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .post("https://osu.ppy.sh/api/v2/chat/new")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "target_id": target_id,
                "message": message,
                "is_action": is_action,
                "uuid": uuid
            }))
            .send()
            .await?;

        let response = check_res(res)?;

        let pm_response: CreateNewPMResponse = response.json().await?;

        Ok(pm_response)
    }

    async fn get_updates(
        &self,
        history_since: Option<u64>,
        includes: Option<Vec<String>>,
        since: Option<u64>,
    ) -> Result<GetUpdatesResponse> {
        println!("ReqwestChat get_updates");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/chat/updates")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("history_since", history_since.map(|s| s.to_string())),
                ("includes", includes.map(|s| s.join(","))),
                ("since", since.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let updates: GetUpdatesResponse = response.json().await?;

        Ok(updates)
    }

    async fn get_channel_messages(
        &self,
        channel: String,
        limit: Option<u64>,
        since: Option<u64>,
        until: Option<u64>,
    ) -> Result<Vec<ChatMessage>> {
        println!("ReqwestChat get_channel_messages");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/chat/channels/{}/messages",
                channel
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("limit", limit.map(|s| s.to_string())),
                ("since", since.map(|s| s.to_string())),
                ("until", until.map(|s| s.to_string())),
            ])
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
        println!("ReqwestChat send_message_to_channel");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .post(format!(
                "https://osu.ppy.sh/api/v2/chat/channels/{}/messages",
                channel
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "message": message,
                "is_action": is_action
            }))
            .send()
            .await?;
        let response = check_res(res)?;
        let message: ChatMessage = response.json().await?;
        Ok(message)
    }

    async fn join_channel(&self, channel: String, user: String) -> Result<ChatChannel> {
        println!("ReqwestChat join_channel");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .put(format!(
                "https://osu.ppy.sh/api/v2/chat/channels/{}/users/{}",
                channel, user
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let response = check_res(res)?;
        let channel: ChatChannel = response.json().await?;
        Ok(channel)
    }

    async fn leave_channel(&self, channel: String, user: String) -> Result<()> {
        println!("ReqwestChat leave_channel");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .delete(format!(
                "https://osu.ppy.sh/api/v2/chat/channels/{}/users/{}",
                channel, user
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let response = check_res(res)?;
        if response.status() == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }
    async fn mark_channel_as_read(
        &self,
        channel: String,
        message: String,
        channel_id: String,
        message_id: String,
    ) -> Result<()> {
        println!("ReqwestChat mark_channel_as_read");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .put(format!(
                "https://osu.ppy.sh/api/v2/chat/channels/{}/mark-as-read/{}",
                channel, message
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("channel_id", channel_id), ("message_id", message_id)])
            .send()
            .await?;
        let response = check_res(res)?;
        if response.status() == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }
    async fn get_channel_list(&self) -> Result<Vec<ChatChannel>> {
        println!("ReqwestChat get_channel_list");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/chat/channels")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let response = check_res(res)?;
        let channels: Vec<ChatChannel> = response.json().await?;
        Ok(channels)
    }
    async fn create_channel(&self, params: CreateChannelParams) -> Result<ChatChannel> {
        println!("ReqwestChat create_channel");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .post("https://osu.ppy.sh/api/v2/chat/channels")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&params)
            .send()
            .await?;
        let response = check_res(res)?;
        let channel: ChatChannel = response.json().await?;
        Ok(channel)
    }
    async fn get_channel(&self, channel: String) -> Result<GetChannelResponse> {
        println!("ReqwestChat get_channel");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/chat/channels/{}",
                channel
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let response = check_res(res)?;
        let channel: GetChannelResponse = response.json().await?;
        Ok(channel)
    }
}
