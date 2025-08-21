use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::news::INews;
use crate::v2::model::news::structs::news_listing::NewsListing;
use crate::v2::model::news::structs::news_post::NewsPost;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooNews {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl INews for GlooNews {
    async fn get_news_listing(
        &self,
        limit: Option<i32>,
        year: Option<i32>,
        cursor_string: Option<String>,
    ) -> Result<NewsListing> {
        console::log_1(&JsValue::from_str("GlooNews get_news_listing"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(year) = year {
            params.push(("year", year.to_string()));
        }
        if let Some(cursor_string) = cursor_string {
            params.push(("cursor_string", cursor_string));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/news?{}",
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
        let news_listing: NewsListing = response.json().await?;

        Ok(news_listing)
    }

    async fn get_news_post(&self, news: String, key: Option<String>) -> Result<NewsPost> {
        console::log_1(&JsValue::from_str("GlooNews get_news_post"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [("key", key.map(|x| x.to_string()))];

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/news/{}?{}",
            proxy_url,
            news,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let news_post: NewsPost = response.json().await?;

        Ok(news_post)
    }
}
