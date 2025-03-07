use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Failtimes{
    fail: Vec<u32>,
    exit: Vec<u32>,
}