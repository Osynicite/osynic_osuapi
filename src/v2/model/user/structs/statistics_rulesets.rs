// src/structs/statistics_rulesets.rs

use serde::{Serialize, Deserialize};
use crate::v2::model::user::structs::statistics::Statistics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsRulesets {
    pub osu: Statistics,
    pub taiko: Statistics,
    pub fruits: Statistics,
    pub mania: Statistics,
}

impl Default for StatisticsRulesets {
    fn default() -> Self {
        StatisticsRulesets {
            osu: Statistics::default(),
            taiko: Statistics::default(),
            fruits: Statistics::default(),
            mania: Statistics::default(),
        }
    }
}