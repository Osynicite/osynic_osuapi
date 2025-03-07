
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Status {
    #[serde(rename = "ranked")]
    Ranked,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "qualified")]
    Qualified,
    #[serde(rename = "loved")]
    Loved,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "wip")]
    WIP,
    #[serde(rename = "graveyard")]
    Graveyard,
}