use super::api::beatmap::GlooBeatmap;
use super::api::multiplayer::GlooMultiplayer;
use super::api::replay::GlooReplay;
use super::api::scores::GlooScores;
use super::api::user::GlooUser;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct OsynicOsuApiV1GlooClient {
    pub beatmap: GlooBeatmap,
    pub user: GlooUser,
    pub multiplayer: GlooMultiplayer,
    pub replay: GlooReplay,
    pub score: GlooScores,
    pub api_key: Arc<Mutex<String>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl OsynicOsuApiV1GlooClient {
    pub fn new(api_key: String) -> Self {
        let api_key = Arc::new(Mutex::new(api_key));
        let proxy_url = Arc::new(Mutex::new(String::new()));
        OsynicOsuApiV1GlooClient {
            beatmap: GlooBeatmap {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            user: GlooUser {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            multiplayer: GlooMultiplayer {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            replay: GlooReplay {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            score: GlooScores {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            api_key,
            proxy_url,
        }
    }

    /// Set the proxy URL for all API calls
    pub fn set_proxy_url(&self, proxy_url: String) {
        let mut url = self.proxy_url.lock().unwrap();
        *url = proxy_url;
    }

    pub fn set_api_key(&self, api_key: String) {
        let mut token = self.api_key.lock().unwrap();
        *token = api_key;
    }
}

impl Default for OsynicOsuApiV1GlooClient {
    fn default() -> Self {
        let api_key = Arc::new(Mutex::new(String::new()));
        let proxy_url = Arc::new(Mutex::new(String::new()));
        OsynicOsuApiV1GlooClient {
            beatmap: GlooBeatmap {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            user: GlooUser {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            multiplayer: GlooMultiplayer {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            replay: GlooReplay {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            score: GlooScores {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            api_key,
            proxy_url,
        }
    }
}
