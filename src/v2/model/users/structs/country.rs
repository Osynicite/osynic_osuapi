// src/structs/country.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

impl Default for Country {
    fn default() -> Self {
        Country {
            code: "".to_string(),
            name: "".to_string(),
        }
    }
}