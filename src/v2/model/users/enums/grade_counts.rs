// src/enums/grade_counts.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
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