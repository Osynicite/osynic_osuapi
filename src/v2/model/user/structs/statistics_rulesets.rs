// src/structs/statistics_rulesets.rs

use serde::{Serialize, Deserialize};
use crate::v2::model::user::structs::statistics::Statistics;

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct StatisticsRulesets {
    pub osu: Statistics,
    pub taiko: Statistics,
    pub fruits: Statistics,
    pub mania: Statistics,
}
