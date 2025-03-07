use crate::v2::model::oauth::structs::o_token::OToken;
use super::api::oauth::ReqwestOauth;
use super::api::users::ReqwestUsers;
use super::api::beatmapsets::ReqwestBeatmapsets;
use super::api::beatmaps::ReqwestBeatmaps;
use super::api::events::ReqwestEvents;
use super::api::search::ReqwestSearch;
use super::api::scores::ReqwestScores;
use super::api::wiki::ReqwestWiki;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct OsynicOsuApiV2Client {
    pub oauth: ReqwestOauth,
    pub users: ReqwestUsers,
    pub beatmapsets: ReqwestBeatmapsets,
    pub beatmaps: ReqwestBeatmaps,
    pub events: ReqwestEvents,
    pub search: ReqwestSearch,
    pub scores: ReqwestScores,
    pub wiki: ReqwestWiki,
    pub o_token: Arc<RwLock<OToken>>,
    pub client: reqwest::Client,
}

impl OsynicOsuApiV2Client {
    pub fn new(o_token: OToken) -> Self {
        let client = reqwest::Client::new();
        let o_token = Arc::new(RwLock::new(o_token));
        OsynicOsuApiV2Client {
            users: ReqwestUsers {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            oauth: ReqwestOauth {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            beatmapsets: ReqwestBeatmapsets {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            beatmaps: ReqwestBeatmaps {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            events: ReqwestEvents {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            search: ReqwestSearch {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            scores: ReqwestScores {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            wiki: ReqwestWiki {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            o_token,
            client,
        }
    }

    pub async fn set_o_token(&self, o_token: OToken) {
        let mut token = self.o_token.write().await;
        *token = o_token;

    }
}

impl Default for OsynicOsuApiV2Client {
    fn default() -> Self {
        let client = reqwest::Client::new();
        let o_token = Arc::new(RwLock::new(OToken::default()));
        OsynicOsuApiV2Client {
            users: ReqwestUsers {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            oauth: ReqwestOauth {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            beatmapsets: ReqwestBeatmapsets {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            beatmaps: ReqwestBeatmaps {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            
            events: ReqwestEvents {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            search: ReqwestSearch {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            scores: ReqwestScores {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            wiki: ReqwestWiki {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            o_token,
            client,
        }
    }
}