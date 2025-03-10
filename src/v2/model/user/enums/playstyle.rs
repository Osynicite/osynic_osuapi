// src/enums/playstyle.rs

use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
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
