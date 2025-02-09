// src/structs/kudosu.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kudosu {
    pub available: u64,
    pub total: u64,
}

impl Default for Kudosu {
    fn default() -> Self {
        Kudosu {
            available: 0,
            total: 0,
        }
    }
}