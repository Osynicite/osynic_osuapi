use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::comments::IComments;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::comment::structs::bundle::CommentBundle;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestComments {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IComments for ReqwestComments {
    async fn get_comments(
        &self,
        after: Option<String>,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        cursor: Option<String>,
        parent_id: Option<String>,
        sort: Option<String>
    ) -> Result<CommentBundle> {
        println!("ReqwestComments get_comments");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .get("https://osu.ppy.sh/api/v2/comments")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("after", after.map(|s| s.to_string())),
                ("commentable_type", commentable_type.map(|s| s.to_string())),
                ("commentable_id", commentable_id.map(|s| s.to_string())),
                ("cursor", cursor.map(|s| s.to_string())),
                ("parent_id", parent_id.map(|s| s.to_string())),
                ("sort", sort.map(|s| s.to_string())),
            ])
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)
    }
}
