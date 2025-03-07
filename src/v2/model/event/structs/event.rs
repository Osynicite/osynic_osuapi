use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Event {
    pub created_at: String,
    #[serde(rename = "createAt")]
    pub created_att: String,
    pub id: u64,
    #[serde(rename = "type")]
    pub rank_type: String,
    #[serde(rename = "scoreRank")]
    pub score_rank: String,
    pub rank: u32,
    pub mode: String,
    pub beatmap: Beatmap,
    pub user: User,
}

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Beatmap {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
}