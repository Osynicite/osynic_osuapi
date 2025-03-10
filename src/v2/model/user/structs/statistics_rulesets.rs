// src/structs/statistics_rulesets.rs

use crate::v2::model::user::structs::statistics::Statistics;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticsRulesets {
    pub osu: Statistics,
    pub taiko: Statistics,
    pub fruits: Statistics,
    pub mania: Statistics,
}
