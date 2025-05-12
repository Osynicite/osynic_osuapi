use crate::error::Result;
use crate::v1::model::multiplayer::MultiplayerScore;
use crate::v2::model::score::structs::score::Score;

pub trait IChat {
    fn chat_keepalive(
        &self,
        history_since: Option<u64>,
        since: Option<u64>,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;

    fn create_new_pm(
        &self,
        target_id: u64,
        message: String,
        is_action: bool,
        uuid: Option<String>,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn get_updates(
        &self,
        history_since: Option<u64>,
        includes: Option<Vec<String>>,
        since: Option<u64>,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;

    fn get_channel_messages(
        &self,
        limit: Option<u64>,
        since: Option<u64>,
        until: Option<u64>,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;

    fn send_message_to_channel(
        &self,
        channel: u64,
        message: String,
        is_action: bool,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn join_channel(
        &self,
        channel: String,
        user: String,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn leave_channel(
        &self,
        channel: String,
        user: String,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn mark_channel_as_read(
        &self,
        channel: String,
        message: String,
        channel_id: String,
        message_id: String,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn get_channel_list(
        &self,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn create_channel(
        &self,
        params: String,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;

    fn get_channel(
        &self,
        channel: String,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;
}
