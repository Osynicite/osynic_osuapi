// src/structs/kudosu.rs
use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Kudosu {
    pub available: u32,
    pub total: u32,
}