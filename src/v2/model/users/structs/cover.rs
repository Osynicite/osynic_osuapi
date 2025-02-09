// src/structs/cover.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}

impl Default for Cover {
    fn default() -> Self {
        Cover {
            custom_url: None,
            url: "".to_string(),
            id: None,
        }
    }
}