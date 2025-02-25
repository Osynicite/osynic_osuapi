// src/structs/replays_watched_count.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaysWatchedCount {
    pub start_date: String,
    pub count: u32,
}