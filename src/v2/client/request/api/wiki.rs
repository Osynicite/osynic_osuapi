use crate::error::Result;
use crate::v2::interface::wiki::IWiki;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::wiki::WikiPage;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReqwestWiki {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IWiki for ReqwestWiki {
    async fn get_wiki_page(&self, locale: String, path: String) -> Result<WikiPage> {
        println!("ReqwestWiki get_beatmapset");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/wiki/{}/{}",
                locale, path
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let wiki: WikiPage = response.json().await?;

        Ok(wiki)
    }
}
