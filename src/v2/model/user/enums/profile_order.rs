// src/enums/profile_order.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum ProfileOrder {
    #[serde(rename = "me")]
    Me,
    #[serde(rename = "recent_activity")]
    RecentActivity,
    #[serde(rename = "top_ranks")]
    TopRanks,
    #[serde(rename = "medals")]
    Medals,
    #[serde(rename = "historical")]
    Historical,
    #[serde(rename = "beatmaps")]
    Beatmaps,
    #[serde(rename = "kudosu")]
    Kudosu,
}