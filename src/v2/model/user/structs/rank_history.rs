// src/structs/rank_history.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankHistory {
    pub mode: String,
    pub data: Vec<u32>,
}
