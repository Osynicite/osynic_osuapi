use serde::{Serialize, Deserialize};

use super::score::Score;

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Scores {
    scores: Vec<Score>,
}