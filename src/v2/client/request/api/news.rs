use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::news::INews;
use crate::v2::model::news::dtos::response::GetNewsListingResponse;
use crate::v2::model::news::structs::news::News;
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestNews {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl INews for ReqwestNews {
    async fn get_news_listing(
        &self,
        limit: Option<u32>,
        year: Option<u32>,
        cursor_string: Option<String>,
    ) -> Result<GetNewsListingResponse> {
        println!("ReqwestNews get_news_listing");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/news")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("limit", limit.map(|x| x.to_string())),
                ("year", year.map(|s| s.to_string())),
                ("cursor", cursor_string.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let news_listing: GetNewsListingResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let news: GetnewsResponse = serde_json::from_str(&text)?;

        Ok(news_listing)
    }
    async fn get_news_post(&self, news: String, key: Option<String>) -> Result<News> {
        println!("Reqwestnews get_news");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/news/{}", news))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("key", key.map(|x| x.to_string()))])
            .send()
            .await?;

        let response = check_res(res)?;

        let news: News = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let matchh: GetMatchResponse = serde_json::from_str(&text)?;

        Ok(news)
    }
}
