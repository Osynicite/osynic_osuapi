// src/structs/cover.rs
use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Cover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}