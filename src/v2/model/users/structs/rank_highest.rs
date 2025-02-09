// src/structs/rank_highest.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankHighest {
    pub rank: u64,
    
    
    pub updated_at: String,
}

impl Default for RankHighest {
    fn default() -> Self {
        RankHighest {
            rank: 0,
            updated_at: "".to_string(),
        }
    }
}