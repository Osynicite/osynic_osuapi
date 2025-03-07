use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Team {
    pub flag_url: String,
    pub id: u32,
    pub name: String,
    pub short_name: String,
}