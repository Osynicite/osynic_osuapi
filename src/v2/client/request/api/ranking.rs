use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::ranking::IRanking;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::ranking::enums::ranking_type::RankingType;
use crate::v2::model::ranking::structs::rankings::{KudosuRankings, Rankings};
use crate::v2::model::ranking::structs::spotlights::Spotlights;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestRanking {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IRanking for ReqwestRanking {
    async fn get_kudosu_ranking(&self, page: Option<u32>) -> Result<KudosuRankings> {
        println!("ReqwestRanking get_kudosu_ranking");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/rankings/kudosu")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("page", page.map(|x| x.to_string()))])
            .send()
            .await?;
        let response = check_res(res)?;
        let kudosu_rankings: KudosuRankings = response.json().await?;
        Ok(kudosu_rankings)
    }

    async fn get_ranking(
        &self,
        mode: Mode,
        ranking_type: RankingType,
        country: Option<String>,
        cursor_string: Option<String>,
        filter: Option<String>,
        spotlight: Option<String>,
        variant: Option<String>,
    ) -> Result<Rankings> {
        println!("ReqwestRankings get_ranking");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/rankings/{}/{}",
                mode.to_ruleset(),
                ranking_type.to_ranking_type()
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("country", country.map(|s| s.to_string())),
                ("cursor", cursor_string.map(|s| s.to_string())),
                ("filter", filter.map(|s| s.to_string())),
                ("spotlight", spotlight.map(|s| s.to_string())),
                ("variant", variant.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let rankings: Rankings = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let rankings: Rankings = serde_json::from_str(&text)?;

        Ok(rankings)
    }

    async fn get_spotlights(&self) -> Result<Spotlights> {
        println!("ReqwestRanking get_spotlights");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/spotlights")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let response = check_res(res)?;
        let spotlights: Spotlights = response.json().await?;
        Ok(spotlights)
    }
}
