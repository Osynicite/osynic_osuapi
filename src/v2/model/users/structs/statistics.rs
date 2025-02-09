// src/structs/statistics.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub count_100: u64,
    pub count_300: u64,
    pub count_50: u64,
    pub count_miss: u64,
    pub country_rank: Option<u64>,
    pub level: Level,
    pub global_rank: u64,
    pub global_rank_exp: Option<u64>,
    pub pp: f64,
    pub pp_exp: Option<f64>,
    pub ranked_score: u64,
    pub hit_accuracy: f64,
    pub play_count: u64,
    pub play_time: u64,
    pub total_score: u64,
    pub total_hits: u64,
    pub maximum_combo: u64,
    pub replays_watched_by_others: u64,
    pub is_ranked: bool,
    pub grade_counts: GradeCounts,
    pub rank: Option<Rank>,
    pub variants: Option<Vec<Variant>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub current: u64,
    pub progress: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeCounts {
    pub ss: u64,
    pub ssh: u64,
    pub s: u64,
    pub sh: u64,
    pub a: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rank {
    pub global: Option<u64>,
    pub country: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub country_rank: u64,
    pub global_rank: u64,
    pub mode: String,
    pub pp: f64,
    pub variant: String,
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            count_100: 0,
            count_300: 0,
            count_50: 0,
            count_miss: 0,
            level: Level {
                current: 0,
                progress: 0,
            },
            global_rank: 0,
            global_rank_exp: None,
            pp: 0.0,
            pp_exp: None,
            ranked_score: 0,
            hit_accuracy: 0.0,
            play_count: 0,
            play_time: 0,
            total_score: 0,
            total_hits: 0,
            maximum_combo: 0,
            replays_watched_by_others: 0,
            is_ranked: false,
            grade_counts: GradeCounts {
                ss: 0,
                ssh: 0,
                s: 0,
                sh: 0,
                a: 0,
            },
            country_rank: None,
            rank: Some(Rank {
                global: None,
                country: 0,
            }),
            variants: None,
        }
    }
}
