use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Cursor{
    pub approved_date: u64,
    pub id: u32,
}