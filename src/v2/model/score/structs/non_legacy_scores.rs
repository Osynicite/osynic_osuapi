use serde::{Deserialize, Serialize};

use super::non_legacy_score::NonLegacyScore;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonLegacyScores {
    pub scores: Vec<NonLegacyScore>,
}
