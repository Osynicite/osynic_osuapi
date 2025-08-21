use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::comments::IComments;
use crate::v2::model::comment::structs::comment_bundle::CommentBundle;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooComments {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IComments for GlooComments {
    async fn get_comments(
        &self,
        commentable_type: String,
        commentable_id: u32,
        cursor: Option<String>,
        parent_id: Option<u32>,
        sort: Option<String>,
    ) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments get_comments"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = vec![
            ("commentable_type", commentable_type),
            ("commentable_id", commentable_id.to_string()),
        ];
        
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        if let Some(parent_id) = parent_id {
            params.push(("parent_id", parent_id.to_string()));
        }
        if let Some(sort) = sort {
            params.push(("sort", sort));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/comments?{}",
            proxy_url,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let comments: CommentBundle = response.json().await?;

        Ok(comments)
    }
}
