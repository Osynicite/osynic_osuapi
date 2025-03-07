use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone,Eq, PartialEq,Serialize, Deserialize)]
pub struct Owner{
    pub id: u32,
    pub username: String,
}