use serde::{Deserialize, Serialize};

use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapPlaycount {
    pub beatmap_id: u32,
    pub count: u32,
    pub beatmap: Beatmap,
    pub beatmapset: Beatmapset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmap {
    pub beatmapset_id: u32,
    pub difficulty_rating: f32,
    pub id: u32,
    pub mode: String,
    pub status: String,
    pub total_length: u32,
    pub user_id: u32,
    pub version: String,
}
