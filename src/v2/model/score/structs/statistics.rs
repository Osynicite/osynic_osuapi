use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Statistics {
    pub count_100: u32,
    pub count_300: u32,
    pub count_50: u32,
    pub count_geki: Option<u32>,
    pub count_katu: u32,
    pub count_miss: u32,
}