use crate::error::Result;
use crate::v2::interface::beatmaps::IBeatmaps;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::beatmap::structs::beatmaps::Beatmaps;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::beatmap::structs::difficulty_attributes::Attributes;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReqwestBeatmaps {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}


impl IBeatmaps for ReqwestBeatmaps {
    
    async fn get_beatmap(&self,beatmap_id: u32) -> Result<Beatmap> {
        println!("ReqwestBeatmaps get_Beatmap");
        
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        
        let response = self
        .client
        .get(format!("https://osu.ppy.sh/api/v2/beatmaps/{}",beatmap_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

        // println!("{:?}", response);

        let beatmap: Beatmap = response.json().await?;

        Ok(beatmap)
        
    }

    async fn get_beatmap_attributes(&self,beatmap_id:u32,mods:Option<Vec<String>>,ruleset:Option<Mode>,ruleset_id:Option<i32>) -> Result<Attributes> {
        println!("ReqwestBeatmaps get_Beatmap_Attributes");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let response = self.client
        .post(format!("https://osu.ppy.sh/api/v2/beatmaps/{}/attributes",beatmap_id))
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

    async fn get_beatmaps(&self,beatmap_ids:Vec<u32>) -> Result<Beatmaps> {
        println!("ReqwestBeatmaps get_Beatmaps");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        // ids%5B%5D=5000992&ids%5B%5D=4457446,只取前50个

        // let params = beatmap_ids.iter().take(50).map(|id| format!("ids%5B%5D={}",id)).collect::<Vec<String>>().join("&");
        // Vec(&str,&str),左边都是ids[]，右边是id
        let params = beatmap_ids.iter().take(50).enumerate().map(|(_,id)| ("ids[]".to_string(),id.to_string())).collect::<Vec<(String,String)>>();
        // println!("{:?}", params);


        let response = self.client
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