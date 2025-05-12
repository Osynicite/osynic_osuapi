use crate::error::Result;
use crate::v1::model::multiplayer::MultiplayerScore;
use crate::v2::model::score::structs::score::Score;

pub trait IForums {
    fn reply_topic(
        &self,
        topic: String,
        body: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn get_topics_listing(
        &self,
        forum_id: String,
        sort: Option<String>,
        limit: Option<u32>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;
    fn create_topic(
        &self,
        params: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn get_topic_and_posts(
        &self,
        sort: Option<String>,
        limit: Option<u32>,
        start: Option<String>,
        end: Option<String>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn edit_topic(
        &self,
        topic: String,
        topic_title: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn edit_post(
        &self,
        post: String,
        body: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn get_forum_listing(
        &self,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn get_forum_and_topic(
        &self,
        forum: u64,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;

}
