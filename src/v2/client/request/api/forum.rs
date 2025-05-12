use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::forum::IForum;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::forum::structs::forum_post::ForumPost;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestForum {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IForum for ReqwestForum {
    async fn reply_topic(
        &self,
        topic: String,
        body: String,
    ) -> Result<ForumPost> {
        println!("ReqwestForum reply_topic");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .post(format!("https://osu.ppy.sh/api/v2/forums/topics/{}/reply", topic))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "body": body
            }))
            .send().await?;

        let response = check_res(res)?;

        let forum_post: ForumPost = response.json().await?;

        Ok(forum_post)
    }
}
