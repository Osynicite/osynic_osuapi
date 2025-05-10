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
}

impl OsynicOsuApiV1GlooClient {
    pub fn new(api_key: String) -> Self {
        let api_key = Arc::new(Mutex::new(api_key));
        OsynicOsuApiV1GlooClient {
            beatmap: GlooBeatmap {
                api_key: api_key.clone(),
            },
            user: GlooUser {
                api_key: api_key.clone(),
            },
            multiplayer: GlooMultiplayer {
                api_key: api_key.clone(),
            },
            replay: GlooReplay {
                api_key: api_key.clone(),
            },
            score: GlooScores {
                api_key: api_key.clone(),
            },
            api_key,
        }
    }

    pub fn set_api_key(&self, api_key: String) {
        let mut token = self.api_key.lock().unwrap();
        *token = api_key;
    }
}

impl Default for OsynicOsuApiV1GlooClient {
    fn default() -> Self {
        let api_key = Arc::new(Mutex::new(String::new()));
        OsynicOsuApiV1GlooClient {
            beatmap: GlooBeatmap {
                api_key: api_key.clone(),
            },
            user: GlooUser {
                api_key: api_key.clone(),
            },
            multiplayer: GlooMultiplayer {
                api_key: api_key.clone(),
            },
            replay: GlooReplay {
                api_key: api_key.clone(),
            },
            score: GlooScores {
                api_key: api_key.clone(),
            },
            api_key,
        }
    }
}