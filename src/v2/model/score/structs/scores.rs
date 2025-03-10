use serde::{Deserialize, Serialize};

use super::score::Score;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scores {
    pub scores: Vec<Score>,
}
