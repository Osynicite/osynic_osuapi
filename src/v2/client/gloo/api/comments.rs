use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::comments::IComments;
use crate::v2::model::comment::structs::bundle::CommentBundle;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooComments {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IComments for GlooComments {
    async fn get_comments(
        &self,
        after: Option<String>,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        cursor: Option<String>,
        parent_id: Option<String>,
        sort: Option<String>,
    ) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments get_comments"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(after) = after {
            query_params.push(("after", after));
        }
        if let Some(commentable_type) = commentable_type {
            query_params.push(("commentable_type", commentable_type));
        }
        if let Some(commentable_id) = commentable_id {
            query_params.push(("commentable_id", commentable_id));
        }
        if let Some(cursor) = cursor {
            query_params.push(("cursor", cursor));
        }
        if let Some(parent_id) = parent_id {
            query_params.push(("parent_id", parent_id));
        }
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/comments")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/comments?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }

    async fn post_comment(
        &self,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        parent_id: Option<String>,
        message: Option<String>,
    ) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments post_comment"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut form_data = Vec::new();
        if let Some(commentable_type) = commentable_type {
            form_data.push(format!("commentable_type={commentable_type}"));
        }
        if let Some(commentable_id) = commentable_id {
            form_data.push(format!("commentable_id={commentable_id}"));
        }
        if let Some(parent_id) = parent_id {
            form_data.push(format!("parent_id={parent_id}"));
        }
        if let Some(message) = message {
            form_data.push(format!("message={message}"));
        }

        let body = form_data.join("&");
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/comments");

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }

    async fn get_comment(&self, comment: String) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments get_comment"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/comments/{comment}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }

    async fn edit_comment(
        &self,
        comment: String,
        message: Option<String>,
    ) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments edit_comment"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut form_data = Vec::new();
        if let Some(message) = message {
            form_data.push(format!("message={message}"));
        }

        let body = form_data.join("&");
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/comments/{comment}");

        let res = Request::put(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {access_token}"))
            .body(body)?
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }

    async fn delete_comment(&self, comment: String) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments delete_comment"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/comments/{comment}");

        let res = Request::delete(&url)
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }

    async fn add_comment_vote(&self, comment: String) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments add_comment_vote"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/comments/{comment}/vote");

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }

    async fn remove_comment_vote(&self, comment: String) -> Result<CommentBundle> {
        console::log_1(&JsValue::from_str("GlooComments remove_comment_vote"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/comments/{comment}/vote");

        let res = Request::delete(&url)
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let comment_bundle: CommentBundle = response.json().await?;
        Ok(comment_bundle)
    }
}
