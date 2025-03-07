use serde::{Serialize, Deserialize};

use super::score::Score;
use super::beatmap_user_score::BeatmapUserScore;
#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct BeatmapScores {
    pub scores: Vec<Score>,
    pub position: Option<BeatmapUserScore>,
}