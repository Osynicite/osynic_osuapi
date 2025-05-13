use crate::error::Result;
use crate::v2::model::chat::dtos::request::CreateChannelParams;
use crate::v2::model::chat::dtos::response::{
    CreateNewPMResponse, GetChannelResponse, GetUpdatesResponse,
};
use crate::v2::model::chat::structs::channel::ChatChannel;
use crate::v2::model::chat::structs::message::ChatMessage;
use crate::v2::model::chat::structs::silences::Silences;

pub trait IChat {
    fn chat_keepalive(
        &self,
        history_since: Option<u64>,
        since: Option<u64>,
    ) -> impl std::future::Future<Output = Result<Silences>> + Send;

    fn create_new_pm(
        &self,
        target_id: u64,
        message: String,
        is_action: bool,
        uuid: Option<String>,
    ) -> impl std::future::Future<Output = Result<CreateNewPMResponse>> + Send;

    fn get_updates(
        &self,
        history_since: Option<u64>,
        includes: Option<Vec<String>>,
        since: Option<u64>,
    ) -> impl std::future::Future<Output = Result<GetUpdatesResponse>> + Send;

    fn get_channel_messages(
        &self,
        channel: String,
        limit: Option<u64>,
        since: Option<u64>,
        until: Option<u64>,
    ) -> impl std::future::Future<Output = Result<Vec<ChatMessage>>> + Send;

    fn send_message_to_channel(
        &self,
        channel: u64,
        message: String,
        is_action: bool,
    ) -> impl std::future::Future<Output = Result<ChatMessage>> + Send;

    fn join_channel(
        &self,
        channel: String,
        user: String,
    ) -> impl std::future::Future<Output = Result<ChatChannel>> + Send;

    fn leave_channel(
        &self,
        channel: String,
        user: String,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    fn mark_channel_as_read(
        &self,
        channel: String,
        message: String,
        channel_id: String,
        message_id: String,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    fn get_channel_list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ChatChannel>>> + Send;

    fn create_channel(
        &self,
        params: CreateChannelParams,
    ) -> impl std::future::Future<Output = Result<ChatChannel>> + Send;

    fn get_channel(
        &self,
        channel: String,
    ) -> impl std::future::Future<Output = Result<GetChannelResponse>> + Send;
}
