// "difficulty_range": {
// 	"max": 6.46,
// 	"min": 4.23
// },

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultyRange {
    pub max: f64,
    pub min: f64,
}
