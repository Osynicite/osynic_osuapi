use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::forum::IForum;
use crate::v2::model::forum::structs::forum_topic::ForumTopic;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooForum {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IForum for GlooForum {
    async fn get_topic_and_posts(
        &self,
        topic_id: u32,
        sort: Option<String>,
        limit: Option<i32>,
        start: Option<String>,
        end: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<ForumTopic> {
        console::log_1(&JsValue::from_str("GlooForum get_topic_and_posts"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(sort) = sort {
            params.push(("sort", sort));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(start) = start {
            params.push(("start", start));
        }
        if let Some(end) = end {
            params.push(("end", end));
        }
        if let Some(cursor_string) = cursor_string {
            params.push(("cursor_string", cursor_string));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/forums/topics/{}?{}",
            proxy_url,
            topic_id,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let forum_topic: ForumTopic = response.json().await?;

        Ok(forum_topic)
    }
}
