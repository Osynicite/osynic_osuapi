use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::model::score::structs::multiplayer::multiplayer_scores::MultiplayerScores;
use crate::v2::interface::multiplayer::IMultiplayer;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::structs::score::Score;
use crate::v2::model::room::structs::room::Room;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestMultiplayer {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IMultiplayer for ReqwestMultiplayer {
    async fn get_user_high_score(
        &self,
        room: String,
        playlist: u64,
        user: u64,
    ) ->Result<Score> {
        println!("ReqwestMultiplayer get_user_high_score");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores/users/{}",
                room, playlist, user
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;

        let score: Score = response.json().await?;

        Ok(score)
    }

    async fn get_scores(
        &self,
        room: String,
        playlist: u64,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<MultiplayerScores> {
        println!("ReqwestMultiplayer get_scores");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores",
                room, playlist
            ))
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

        let scores: MultiplayerScores = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let scores: MultiplayerScores = serde_json::from_str(&text)?;

        Ok(scores)
    }
    
    async fn get_score(
            &self,
            room: String,
            playlist: u64,
            score: u64,
        ) -> Result<Score> {
        println!("ReqwestMultiplayer get_score");
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/rooms/{}/playlist/{}/scores/{}",
                room, playlist, score
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let response = check_res(res)?;
    
        let score: Score = response.json().await?;
    
        Ok(score)
    
    }

    async fn get_multiplayer_rooms(
        &self,
        limit: Option<u32>,
        mode: Option<String>,
        season_id: Option<u32>,
        sort: Option<String>,
        type_group: Option<String>,
    ) -> Result<Vec<Room>> {
        println!("ReqwestMultiplayer get_multiplayer_rooms");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/rooms")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("limit", limit.map(|x| x.to_string())),
                ("mode", mode.map(|s| s.to_string())),
                ("season_id", season_id.map(|s| s.to_string())),
                ("sort", sort.map(|s| s.to_string())),
                ("type_group", type_group.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let rooms:  Vec<Room> = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let rooms: Vec<Room> = serde_json::from_str(&text)?;

        Ok(rooms)
    }
}
