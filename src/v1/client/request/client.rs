use super::api::beatmap::ReqwestBeatmap;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct OsynicOsuApiV1Client {
    pub beatmap: ReqwestBeatmap,
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
            client,
            api_key,
        }
    }
}
