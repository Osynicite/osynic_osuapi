use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::forum::IForum;
use crate::v2::model::forum::dtos::request::CreateTopicParams;
use crate::v2::model::forum::dtos::response::{
    CreateTopicResponse, GetForumAndTopicsResponse, GetTopicAndPostsResponse,
};
use crate::v2::model::forum::structs::forums::Forums;
use crate::v2::model::forum::structs::post::ForumPost;
use crate::v2::model::forum::structs::topic::{ForumTopic, TopicListing};
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooForum {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IForum for GlooForum {
    async fn reply_topic(&self, topic: String, body: String) -> Result<ForumPost> {
        console::log_1(&JsValue::from_str("GlooForum reply_topic"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = vec![format!("body={}", body)];
        let body_content = form_data.join("&");
        let url = format!("{}https://osu.ppy.sh/api/v2/forums/topics/{}/reply", proxy_url, topic);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", access_token))
            .body(body_content)?
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
        console::log_1(&JsValue::from_str("GlooForum get_topics_listing"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(forum_id) = forum_id {
            query_params.push(("forum_id", forum_id));
        }
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(cursor_string) = cursor_string {
            query_params.push(("cursor_string", cursor_string));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/forums/topics", proxy_url)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/forums/topics?{}", proxy_url, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let topic_listing: TopicListing = response.json().await?;
        Ok(topic_listing)
    }

    async fn create_topic(&self, params: CreateTopicParams) -> Result<CreateTopicResponse> {
        console::log_1(&JsValue::from_str("GlooForum create_topic"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let body = serde_json::to_string(&params)?;
        let url = format!("{}https://osu.ppy.sh/api/v2/forums/topics", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .body(body)?
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
        console::log_1(&JsValue::from_str("GlooForum get_topic_and_posts"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(start) = start {
            query_params.push(("start", start));
        }
        if let Some(end) = end {
            query_params.push(("end", end));
        }
        if let Some(cursor_string) = cursor_string {
            query_params.push(("cursor_string", cursor_string));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/forums/topics/{}", proxy_url, topic)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/forums/topics/{}?{}", proxy_url, topic, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let topic_posts_response: GetTopicAndPostsResponse = response.json().await?;
        Ok(topic_posts_response)
    }

    async fn edit_topic(&self, topic: String, topic_title: String) -> Result<ForumTopic> {
        console::log_1(&JsValue::from_str("GlooForum edit_topic"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = vec![format!("forum_topic_title={}", topic_title)];
        let body = form_data.join("&");
        let url = format!("{}https://osu.ppy.sh/api/v2/forums/topics/{}", proxy_url, topic);

        let res = Request::put(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", access_token))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let forum_topic: ForumTopic = response.json().await?;
        Ok(forum_topic)
    }

    async fn edit_post(&self, post: String, body: String) -> Result<ForumPost> {
        console::log_1(&JsValue::from_str("GlooForum edit_post"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = vec![format!("body={}", body)];
        let body_content = form_data.join("&");
        let url = format!("{}https://osu.ppy.sh/api/v2/forums/posts/{}", proxy_url, post);

        let res = Request::put(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", access_token))
            .body(body_content)?
            .send()
            .await?;

        let response = check_res(res)?;
        let forum_post: ForumPost = response.json().await?;
        Ok(forum_post)
    }

    async fn get_forum_listing(&self) -> Result<Forums> {
        console::log_1(&JsValue::from_str("GlooForum get_forum_listing"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{}https://osu.ppy.sh/api/v2/forums", proxy_url);

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let forums: Forums = response.json().await?;
        Ok(forums)
    }

    async fn get_forum_and_topic(&self, forum: u64) -> Result<GetForumAndTopicsResponse> {
        console::log_1(&JsValue::from_str("GlooForum get_forum_and_topic"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{}https://osu.ppy.sh/api/v2/forums/{}", proxy_url, forum);

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let forum_topics_response: GetForumAndTopicsResponse = response.json().await?;
        Ok(forum_topics_response)
    }
}
