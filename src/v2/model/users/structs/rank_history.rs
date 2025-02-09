// src/structs/rank_history.rs

use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankHistory {
    pub mode: String,
    pub data: Vec<u64>,
}

impl Default for RankHistory {
    fn default() -> Self {
        RankHistory {
            mode: "".to_string(),
            data: Vec::new(),
        }
    }
}