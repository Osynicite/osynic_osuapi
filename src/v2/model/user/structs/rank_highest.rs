// src/structs/rank_highest.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankHighest {
    pub rank: u32,
    pub updated_at: String,
}
