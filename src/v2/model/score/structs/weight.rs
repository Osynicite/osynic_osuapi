use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weight {
    pub percentage: f64,
    pub pp: f64,
}
