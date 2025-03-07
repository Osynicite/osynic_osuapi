use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Owner{
    pub id: u32,
    pub username: String,
}