use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Failtimes {
    fail: Vec<u32>,
    exit: Vec<u32>,
}
