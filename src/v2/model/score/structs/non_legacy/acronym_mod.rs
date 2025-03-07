use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct AcronymMod {
    pub acronym: String,
}