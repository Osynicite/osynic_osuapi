use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::forum::IForum;
use crate::v2::model::forum::dtos::request::CreateTopicParams;
use crate::v2::model::forum::dtos::response::{
    CreateTopicResponse, GetForumAndTopicsResponse, GetTopicAndPostsResponse,
};
use crate::v2::model::forum::structs::forums::Forums;
use crate::v2::model::forum::structs::post::ForumPost;
use crate::v2::model::forum::structs::topic::{ForumTopic, TopicListing};
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestForum {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IForum for ReqwestForum {
    async fn reply_topic(&self, topic: String, body: String) -> Result<ForumPost> {
        println!("ReqwestForum reply_topic");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .post(format!(
                "https://osu.ppy.sh/api/v2/forums/topics/{}/reply",
                topic
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "body": body
            }))
            .send()
            .await?;

        let response = check_res(res)?;

        let forum_post: ForumPost = response.json().await?;

        Ok(forum_post)
    }
    async fn get_topics_listing(
        &self,
        forum_id: Option<String>,
        sort: Option<String>,
        limit: Option<u32>,
        cursor_string: Option<String>,
    ) -> Result<TopicListing> {
        println!("ReqwestForum get_topics_listing");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/forums/topics")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("forum_id", forum_id.map(|s| s.to_string())),
                ("sort", sort.map(|s| s.to_string())),
                ("limit", limit.map(|s| s.to_string())),
                ("cursor_string", cursor_string.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let topic_listing: TopicListing = response.json().await?;

        Ok(topic_listing)
    }

    async fn create_topic(&self, params: CreateTopicParams) -> Result<CreateTopicResponse> {
        println!("ReqwestForum create_topic");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .post("https://osu.ppy.sh/api/v2/forums/topics")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&params)
            .send()
            .await?;
        let response = check_res(res)?;
        let create_topic_response: CreateTopicResponse = response.json().await?;
        Ok(create_topic_response)
    }
    async fn get_topic_and_posts(
        &self,
        topic: u32,
        sort: Option<String>,
        limit: Option<u32>,
        start: Option<String>,
        end: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetTopicAndPostsResponse> {
        println!("ReqwestForum get_topic_and_posts");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/forums/topics/{}", topic))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("sort", sort.map(|s| s.to_string())),
                ("limit", limit.map(|s| s.to_string())),
                ("start", start.map(|s| s.to_string())),
                ("end", end.map(|s| s.to_string())),
                ("cursor_string", cursor_string.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let topic_and_posts: GetTopicAndPostsResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let topic_and_posts: GetTopicAndPostsResponse = serde_json::from_str(&text)?;

        Ok(topic_and_posts)
    }

    async fn edit_topic(&self, topic: String, topic_title: String) -> Result<ForumTopic> {
        println!("ReqwestForum edit_topic");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .patch(format!("https://osu.ppy.sh/api/v2/forums/topics/{}", topic))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "forum_topic[topic_title]": topic_title
            }))
            .send()
            .await?;

        let response = check_res(res)?;

        let forum_topic: ForumTopic = response.json().await?;
        Ok(forum_topic)
    }

    async fn edit_post(&self, post: String, body: String) -> Result<ForumPost> {
        println!("ReqwestForum edit_post");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .patch(format!("https://osu.ppy.sh/api/v2/forums/posts/{}", post))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "body": body
            }))
            .send()
            .await?;

        let response = check_res(res)?;

        let forum_post: ForumPost = response.json().await?;

        Ok(forum_post)
    }
    async fn get_forum_listing(&self) -> Result<Forums> {
        println!("ReqwestForum get_forum_listing");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/forums")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;

        let forums: Forums = response.json().await?;

        Ok(forums)
    }

    async fn get_forum_and_topic(&self, forum: u64) -> Result<GetForumAndTopicsResponse> {
        println!("ReqwestForum get_forum_and_topic");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/forums/{}", forum))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;

        let forum_and_topic: GetForumAndTopicsResponse = response.json().await?;

        Ok(forum_and_topic)
    }
}
