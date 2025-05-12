use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::chat::IChat;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::chat::structs::silences::Silences;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestChat {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IChat for ReqwestChat {
    async fn chat_keepalive(
        &self,
        history_since: Option<u64>,
        since: Option<u64>,
    ) -> Result<Silences> {
        println!("ReqwestChat chat_keepalive");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self.client
            .post("https://osu.ppy.sh/api/v2/chat/ack")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("history_since", history_since.map(|s| s.to_string())),
                ("since", since.map(|s| s.to_string())),
            ])
            .send().await?;

        let response = check_res(res)?;

        let silences: Silences = response.json().await?;

        Ok(silences)
    }
}
