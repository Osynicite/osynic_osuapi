use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Genre{
    pub id: u32,
    pub name: String,
}