// src/structs/kudosu.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kudosu {
    pub available: u32,
    pub total: u32,
}

impl Default for Kudosu {
    fn default() -> Self {
        Kudosu {
            available: 0,
            total: 0,
        }
    }
}