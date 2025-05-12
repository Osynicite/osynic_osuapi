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

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let comments: CommentBundle = serde_json::from_str(&text)?;

        Ok(comments)
    }
    async fn post_comment(
        &self,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        parent_id: Option<String>,
        message: Option<String>,
    ) -> Result<CommentBundle>{
        println!("ReqwestComments post_comment");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .post("https://osu.ppy.sh/api/v2/comments")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("comment.commentable_type", commentable_type.map(|s| s.to_string())),
                ("comment.commentable_id", commentable_id.map(|s| s.to_string())),
                ("comment.message", message.map(|s| s.to_string())),
                ("comment.parent_id", parent_id.map(|s| s.to_string())),
            ])
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)

    }
    async fn get_comment(
        &self,
        comment: String,
    ) -> Result<CommentBundle>{
        println!("ReqwestComments get_comment");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .get(format!("https://osu.ppy.sh/api/v2/comments/{}", comment))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)

    }
    async fn edit_comment(
        &self,
        comment: String,
        message: Option<String>,
    ) -> Result<CommentBundle>{
        println!("ReqwestComments edit_comment");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .patch(format!("https://osu.ppy.sh/api/v2/comments/{}", comment))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "message": message,
            }))
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)

    }
    async fn delete_comment(
        &self,
        comment: String,
    ) -> Result<CommentBundle>{
        println!("ReqwestComments delete_comment");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .delete(format!("https://osu.ppy.sh/api/v2/comments/{}", comment))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)

    }
    async fn add_comment_vote(
        &self,
        comment: String,
    ) -> Result<CommentBundle>{
        println!("ReqwestComments add_comment_vote");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .post(format!("https://osu.ppy.sh/api/v2/comments/{}/vote", comment))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)

    }
    async fn remove_comment_vote(
        &self,
        comment: String,
    ) -> Result<CommentBundle>{
        println!("ReqwestComments remove_comment_vote");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .delete(format!("https://osu.ppy.sh/api/v2/comments/{}/vote", comment))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send().await?;

        let response = check_res(res)?;

        let comments: CommentBundle = response.json().await?;

        Ok(comments)

    }
}
