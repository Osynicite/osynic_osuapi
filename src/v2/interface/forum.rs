use crate::error::Result;
use crate::v2::model::forum::dtos::request::CreateTopicParams;
use crate::v2::model::forum::dtos::response::{
    CreateTopicResponse, GetForumAndTopicsResponse, GetTopicAndPostsResponse,
};
use crate::v2::model::forum::structs::forums::Forums;
use crate::v2::model::forum::structs::post::ForumPost;
use crate::v2::model::forum::structs::topic::{ForumTopic, TopicListing};

pub trait IForum {
    fn reply_topic(
        &self,
        topic: String,
        body: String,
    ) -> impl std::future::Future<Output = Result<ForumPost>>;
    fn get_topics_listing(
        &self,
        forum_id: Option<String>,
        sort: Option<String>,
        limit: Option<u32>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<TopicListing>>;
    fn create_topic(
        &self,
        params: CreateTopicParams,
    ) -> impl std::future::Future<Output = Result<CreateTopicResponse>>;
    fn get_topic_and_posts(
        &self,
        topic: u32,
        sort: Option<String>,
        limit: Option<u32>,
        start: Option<String>,
        end: Option<String>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<GetTopicAndPostsResponse>>;
    fn edit_topic(
        &self,
        topic: String,
        topic_title: String,
    ) -> impl std::future::Future<Output = Result<ForumTopic>>;
    fn edit_post(
        &self,
        post: String,
        body: String,
    ) -> impl std::future::Future<Output = Result<ForumPost>>;
    fn get_forum_listing(&self) -> impl std::future::Future<Output = Result<Forums>>;
    fn get_forum_and_topic(
        &self,
        forum: u64,
    ) -> impl std::future::Future<Output = Result<GetForumAndTopicsResponse>>;
}
