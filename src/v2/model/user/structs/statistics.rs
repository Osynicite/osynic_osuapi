// src/structs/statistics.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub count_100: u32,
    pub count_300: u32,
    pub count_50: u32,
    pub count_miss: u32,
    pub country_rank: Option<u32>,
    pub level: Level,
    pub global_rank: Option<u32>,
    pub global_rank_exp: Option<u32>,
    pub pp: f64,
    pub pp_exp: Option<f64>,
    pub ranked_score: u64,
    pub hit_accuracy: f64,
    pub play_count: u32,
    pub play_time: u32,
    pub total_score: u64,
    pub total_hits: u64,
    pub maximum_combo: u32,
    pub replays_watched_by_others: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_change_since_30_days: Option<i32>,
    pub is_ranked: bool,
    pub grade_counts: GradeCounts,
    pub rank: Option<Rank>,
    pub variants: Option<Vec<Variant>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub current: u32,
    pub progress: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GradeCounts {
    pub ss: u32,
    pub ssh: u32,
    pub s: u32,
    pub sh: u32,
    pub a: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rank {
    pub global: Option<u32>,
    pub country: u32,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub country_rank: u32,
    pub global_rank: u32,
    pub mode: String,
    pub pp: f64,
    pub variant: String,
}
