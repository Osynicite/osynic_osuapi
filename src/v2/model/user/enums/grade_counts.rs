// src/enums/grade_counts.rs

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Grade {
    #[serde(rename = "ss")]
    SS,
    #[serde(rename = "ssh")]
    SSH,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "sh")]
    SH,
    #[serde(rename = "a")]
    A,
}
