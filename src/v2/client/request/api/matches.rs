use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::matches::IMatches;
use crate::v2::model::matches::dtos::response::{GetMatchResponse, GetMatchesListingResponse};
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestMatches {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IMatches for ReqwestMatches {
    async fn get_matches_listing(
        &self,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetMatchesListingResponse> {
        println!("ReqwestMatches get_Matches");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/matches")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("limit", limit.map(|x| x.to_string())),
                ("sort", sort.map(|s| s.to_string())),
                ("cursor", cursor_string.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let matches_listing: GetMatchesListingResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let Matches: GetMatchesResponse = serde_json::from_str(&text)?;

        Ok(matches_listing)
    }
    async fn get_match(
        &self,
        match_id: u64,
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    ) -> Result<GetMatchResponse> {
        println!("ReqwestMatches get_Matches");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/matches/{}", match_id))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("before", before.map(|x| x.to_string())),
                ("after", after.map(|x| x.to_string())),
                ("limit", limit.map(|x| x.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let matchh: GetMatchResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let matchh: GetMatchResponse = serde_json::from_str(&text)?;

        Ok(matchh)
    }
}
