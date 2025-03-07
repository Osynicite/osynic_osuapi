use crate::error::Result;
use crate::v2::interface::scores::IScores;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::dtos::response::GetScoresResponse;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReqwestScores {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}


impl IScores for ReqwestScores {
    

    async fn get_scores(&self, ruleset: Option<Mode>,cursor_string: Option<String>) -> Result<GetScoresResponse> {
        println!("ReqwestScores get_beatmapset");
        
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        
        let response = self
            .client
            .get("https://osu.ppy.sh/api/v2/scores")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("ruleset",ruleset.map(|x| x.to_ruleset())),
                ("cursor",cursor_string.map(|x| x.to_string()))
            ])
            .send()
            .await?;

        let scores: GetScoresResponse = response.json().await?;

        Ok(scores)
        
    }
}