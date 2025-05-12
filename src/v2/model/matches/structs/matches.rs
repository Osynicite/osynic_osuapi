use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub id: u64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub name: String,
}
