use crate::error::Result;
use crate::v2::interface::beatmapsets::IBeatmapsets;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::search::dtos::params::BeatmapsetsSearchParams;
use crate::v2::model::search::dtos::response::BeatmapsetsSearchResponse;
use tokio::io::AsyncWriteExt;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReqwestBeatmapsets {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}


impl IBeatmapsets for ReqwestBeatmapsets {
    async fn search(
        &self,
        params: BeatmapsetsSearchParams,
    ) -> Result<BeatmapsetsSearchResponse> {
        println!("ReqwestBeatmapsets search");
        
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        // ?e=storyboard&c=recommended.converts.follows.spotlights.featured_artists&g=5&m=0&nsfw=true&played=unplayed&q=213&r=X.SH.XH.C.B&s=any
        // let params = [
        //     ("q", "1"),
        //     ("c","recommended.converts.follows.spotlights.featured_artists"),
        //     ("m","0"),
        //     ("s","any"),
        //     ("played","unplayed"),
        //     ("nsfw","true"),
        //     ("e","storyboard"),
        //     ("r","X.SH.XH.C.B"),
        //     ("g","5"),
        // ];

        let params = params.build_params();


        // 1. [x] 搜索参数选项封装
        // 2. [ ] 写好谱面结构体的解析，就能用了

        let response = self
            .client
            .get("https://osu.ppy.sh/api/v2/beatmapsets/search")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&params)
            .send()
            .await?;

        println!("{:?}", response);

        // 解析成结构体
        let beatmapsets_search_response: BeatmapsetsSearchResponse = response.json().await?;

        Ok(beatmapsets_search_response)
    }

    async fn download(&self,beatmapset_id: u32) -> Result<()> {
        println!("ReqwestBeatmapsets download");
        
        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        
        let response = self
        .client
        .get(format!("https://osu.ppy.sh/api/v2/beatmapsets/{}/download",beatmapset_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    // println!("{:?}", response);
    // 把文件写入本地
    let mut file = tokio::fs::File::create(format!("{}.osz",beatmapset_id)).await?;
    let bytes = response.bytes().await?;
    file.write_all(&bytes).await?;
    // {"authentication":"basic"}
        
        Ok(())
    }
}