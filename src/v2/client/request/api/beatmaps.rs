use crate::error::Result;
use crate::v2::interface::beatmaps::IBeatmaps;
use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::beatmap::structs::beatmaps::Beatmaps;
use crate::v2::model::beatmap::structs::difficulty_attributes::Attributes;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::structs::beatmap_scores::BeatmapScores;
use crate::v2::model::score::structs::beatmap_user_score::BeatmapUserScore;
use crate::v2::model::score::structs::non_legacy_scores::NonLegacyScores;
use crate::v2::model::score::structs::scores::Scores;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestBeatmaps {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IBeatmaps for ReqwestBeatmaps {
    async fn get_user_score(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
    ) -> Result<BeatmapUserScore> {
        println!("ReqwestBeatmaps get_User_Score");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/beatmaps/{}/scores/users/{}",
                beatmap_id, user_id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("legacy", legacy_only.map(|x| x.to_string())),
                ("mode", mode.map(|x| x.to_ruleset())),
                ("mods", mods),
            ])
            .send()
            .await?;

        let user_score: BeatmapUserScore = response.json().await?;

        Ok(user_score)
    }

    async fn get_user_scores(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        ruleset: Option<Mode>,
    ) -> Result<Scores> {
        println!("ReqwestBeatmaps get_User_Scores");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/beatmaps/{}/scores/users/{}/all",
                beatmap_id, user_id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("legacy", legacy_only.map(|x| x.to_string())),
                ("mode", mode.map(|x| x.to_ruleset())),
                ("ruleset", ruleset.map(|x| x.to_ruleset())),
            ])
            .send()
            .await?;

        let scores: Scores = response.json().await?;

        Ok(scores)
    }

    async fn get_scores(
        &self,
        beatmap_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> Result<BeatmapScores> {
        println!("ReqwestBeatmaps get_Scores");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/beatmaps/{}/scores",
                beatmap_id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("legacy", legacy_only.map(|x| x.to_string())),
                ("mode", mode.map(|x| x.to_ruleset())),
                ("mods", mods),
                ("type", ranking_type),
            ])
            .send()
            .await?;

        let scores: BeatmapScores = response.json().await?;

        Ok(scores)
    }

    async fn get_solo_scores(
        &self,
        beatmap_id: u32,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> Result<NonLegacyScores> {
        println!("ReqwestBeatmaps get_Solo_Scores");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/beatmaps/{}/solo-scores",
                beatmap_id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("mode", mode.map(|x| x.to_ruleset())),
                ("mods", mods),
                ("type", ranking_type),
            ])
            .send()
            .await?;

        let scores: NonLegacyScores = response.json().await?;

        Ok(scores)
    }

    async fn get_beatmap(&self, beatmap_id: u32) -> Result<Beatmap> {
        println!("ReqwestBeatmaps get_Beatmap");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/beatmaps/{}", beatmap_id))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        // println!("{:?}", response);

        let beatmap: Beatmap = response.json().await?;

        Ok(beatmap)
    }

    async fn get_beatmap_attributes(
        &self,
        beatmap_id: u32,
        mods: Option<Vec<String>>,
        ruleset: Option<Mode>,
        ruleset_id: Option<i32>,
    ) -> Result<Attributes> {
        println!("ReqwestBeatmaps get_Beatmap_Attributes");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self
            .client
            .post(format!(
                "https://osu.ppy.sh/api/v2/beatmaps/{}/attributes",
                beatmap_id
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "mods": mods,
                "ruleset": ruleset,
                "ruleset_id": ruleset_id
            }))
            .send()
            .await?;

        let attributes: Attributes = response.json().await?;

        Ok(attributes)
    }

    async fn get_beatmaps(&self, beatmap_ids: Vec<u32>) -> Result<Beatmaps> {
        println!("ReqwestBeatmaps get_Beatmaps");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        // ids%5B%5D=5000992&ids%5B%5D=4457446,只取前50个

        // let params = beatmap_ids.iter().take(50).map(|id| format!("ids%5B%5D={}",id)).collect::<Vec<String>>().join("&");
        // Vec(&str,&str),左边都是ids[]，右边是id
        let params = beatmap_ids
            .iter()
            .take(50)
            .enumerate()
            .map(|(_, id)| ("ids[]".to_string(), id.to_string()))
            .collect::<Vec<(String, String)>>();
        // println!("{:?}", params);

        let response = self
            .client
            .get("https://osu.ppy.sh/api/v2/beatmaps")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        let beatmaps: Beatmaps = response.json().await?;

        Ok(beatmaps)
    }
}
