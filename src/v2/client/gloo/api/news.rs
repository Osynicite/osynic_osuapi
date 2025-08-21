use crate::error::Result;
use crate::v2::interface::news::INews;
use crate::v2::model::news::dtos::response::GetNewsListingResponse;
use crate::v2::model::news::structs::news::News;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;
use crate::v2::client::gloo::check::check_res;

#[derive(Clone)]
pub struct GlooNews {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl INews for GlooNews {
    async fn get_news_listing(
        &self,
        limit: Option<u32>,
        year: Option<u32>,
        cursor_string: Option<String>,
    ) -> Result<GetNewsListingResponse> {
        console::log_1(&JsValue::from_str("GlooNews get_news_listing"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(year) = year {
            query_params.push(("year", year.to_string()));
        }
        if let Some(cursor) = cursor_string {
            query_params.push(("cursor_string", cursor));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/news", proxy_url)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/news?{}", proxy_url, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let news_listing: GetNewsListingResponse = response.json().await?;
        Ok(news_listing)
    }

    async fn get_news_post(&self, news_id: String, key: Option<String>) -> Result<News> {
        console::log_1(&JsValue::from_str("GlooNews get_news_post"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(key) = key {
            query_params.push(("key", key));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/news/{}", proxy_url, news_id)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/news/{}?{}", proxy_url, news_id, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let news_post: News = response.json().await?;
        Ok(news_post)
    }
}
