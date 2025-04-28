use crate::error::Result;
use crate::v1::client::request::check::check_res;
use crate::v1::interface::multiplayer::IMultiplayer;
use crate::v1::model::multiplayer::{MultiplayerResponse, GetMatchParams};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestMultiplayer {
    pub client: reqwest::Client,
    pub api_key: Arc<RwLock<String>>,
}

impl IMultiplayer for ReqwestMultiplayer {

    async fn get_match(
        &self,
        params: GetMatchParams,
    ) -> Result<MultiplayerResponse> {
        println!("ReqwestMultiplayer get_match");


        let key = {
            let key = self.api_key.read().await;
            key.clone()
        };

        let params = params.api_key(key)
            .build_params();


        let res = self
            .client
            .get("https://osu.ppy.sh/api/get_match")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .query(&params)
            .send()
            .await?;

        let response = check_res(res)?;

        let multiplayer: MultiplayerResponse  = response.json().await?;

        Ok(multiplayer)

    }
}