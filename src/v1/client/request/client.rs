use super::api::beatmap::ReqwestBeatmap;
use super::api::multiplayer::ReqwestMultiplayer;
use super::api::replay::ReqwestReplay;
use super::api::scores::ReqwestScores;
use super::api::user::ReqwestUser;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct OsynicOsuApiV1Client {
    pub beatmap: ReqwestBeatmap,
    pub user: ReqwestUser,
    pub multiplayer: ReqwestMultiplayer,
    pub replay: ReqwestReplay,
    pub score: ReqwestScores,
    pub api_key: Arc<RwLock<String>>,
    pub client: reqwest::Client,
}

impl OsynicOsuApiV1Client {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        let api_key = Arc::new(RwLock::new(api_key));
        OsynicOsuApiV1Client {
            beatmap: ReqwestBeatmap {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            user: ReqwestUser {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            multiplayer: ReqwestMultiplayer {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            replay: ReqwestReplay {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            score: ReqwestScores {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            client,
            api_key,
        }
    }

    pub async fn set_api_key(&self, api_key: String) {
        let mut token = self.api_key.write().await;
        *token = api_key;
    }
}

impl Default for OsynicOsuApiV1Client {
    fn default() -> Self {
        let client = reqwest::Client::new();
        let api_key = Arc::new(RwLock::new(String::new()));
        OsynicOsuApiV1Client {
            beatmap: ReqwestBeatmap {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            user: ReqwestUser {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            multiplayer: ReqwestMultiplayer {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            replay: ReqwestReplay {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            score: ReqwestScores {
                client: client.clone(),
                api_key: api_key.clone(),
            },
            client,
            api_key,
        }
    }
}
