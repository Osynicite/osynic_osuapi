use serde::{Deserialize, Serialize};

use super::beatmap_user_score::BeatmapUserScore;
use super::score::Score;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapScores {
    pub scores: Vec<Score>,
    pub position: Option<BeatmapUserScore>,
}
