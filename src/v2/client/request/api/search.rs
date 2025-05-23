use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::search::ISearch;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::search::dtos::response::SearchResponse;
use crate::v2::model::search::enums::search_mode::SearchMode;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestSearch {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl ISearch for ReqwestSearch {
    async fn search(
        &self,
        mode: Option<SearchMode>,
        query: Option<String>,
        page: Option<u32>,
    ) -> Result<SearchResponse> {
        println!("ReqwestSearch search");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/search")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("mode", mode.map(|m| m.to_search_param())),
                ("query", query.map(|q| q.to_string())),
                ("page", page.map(|p| p.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;
        let search: SearchResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let search: SearchResponse = serde_json::from_str(&text)?;

        Ok(search)
    }
}
