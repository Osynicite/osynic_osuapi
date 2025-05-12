use serde::{Deserialize, Serialize};

use crate::v2::model::chat::structs::channel::ChatChannel;
use crate::v2::model::chat::structs::message::{ChatMessage, Sender};
use crate::v2::model::chat::structs::user_silence::UserSilence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateNewPMResponse {
    pub channel: ChatChannel,
    pub message: ChatMessage,
    pub new_channel_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUpdatesResponse {
    pub presence: Option<Vec<ChatMessage>>,
    // `messages` field is not used and will be removed.
    pub messages: (), 
    pub silences: Option<Vec<UserSilence>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChannelResponse {
    pub channel: ChatChannel,
    pub users: Vec<Sender>,
}