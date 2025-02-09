// src/enums/playstyle.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum Playstyle {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "keyboard")]
    Keyboard,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "touch")]
    Touch,
}