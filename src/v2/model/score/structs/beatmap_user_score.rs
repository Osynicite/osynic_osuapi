use serde::{Serialize, Deserialize};

use super::score::Score;

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct BeatmapUserScore {
    pub position: Option<u32>,
    pub score: Option<Score>,
    pub error: Option<String>,
}