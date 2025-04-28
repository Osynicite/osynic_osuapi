use crate::error::Result;
use crate::v1::client::request::check::check_res;
use crate::v1::interface::beatmap::IBeatmap;
use crate::v1::model::beatmap::{Beatmap, GetBeatmapsParams};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestBeatmap {
    pub client: reqwest::Client,
    pub api_key: Arc<RwLock<String>>,
}

impl IBeatmap for ReqwestBeatmap {

    async fn get_beatmaps(
        &self,
        params: GetBeatmapsParams,
    ) -> Result<Vec<Beatmap>> {
        println!("ReqwestBeatmap get_beatmaps");

        // let GetBeatmapsParams {
        //     since,
        //     sid,
        //     bid,
        //     uid,
        //     typee,
        //     mode,
        //     has_converted,
        //     hash,
        //     limit,
        //     mods,
        // } = params;

        // let key = {
        //     let key = self.api_key.read().await;
        //     key.clone()
        // };

        // let m_str = mode.map(|x| x.to_string());
        // let a_str = has_converted.map(|x| x.to_string());
        // let limit_str = limit.map(|x| x.to_string());
        // let mods_str = mods.map(|x| x.to_string());

        // let params = [
        //     ("k", Some(key.as_str())),
        //     ("since", since.as_deref()),
        //     ("s", sid.as_deref()),
        //     ("b", bid.as_deref()),
        //     ("u", uid.as_deref()),
        //     ("type", typee.as_deref()),
        //     ("m", m_str.as_deref()),
        //     ("a", a_str.as_deref()),
        //     ("h", hash.as_deref()),
        //     ("limit", limit_str.as_deref()),
        //     ("mods", mods_str.as_deref()),
        // ];

        
        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key).build_params();

        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_beatmaps")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let beatmaps: Vec<Beatmap> = response.json().await?;

        Ok(beatmaps)

    }
}