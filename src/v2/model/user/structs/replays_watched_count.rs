// src/structs/replays_watched_count.rs
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaysWatchedCount {
    pub start_date: String,
    pub count: u32,
}
