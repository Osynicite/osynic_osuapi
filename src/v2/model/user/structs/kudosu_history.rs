use serde::{Deserialize, Serialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct KudosuHisotry {
    pub id: u64,
    pub action: String,
    pub amount: i32,
    pub model: String,
    pub created_at: String,
    pub giver: Option<String>,
    pub post: Post,
    pub details: Details,
}

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Post {
    pub url: String,
    pub title: String,
}

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Details {
    pub event: String,
}