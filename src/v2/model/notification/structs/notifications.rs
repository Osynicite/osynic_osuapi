use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    pub name: String,
    pub created_at: String,
    pub object_type: String,
    pub object_id: u64,
    pub source_user_id: u64,
    pub is_read: bool,
    pub details: Details,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub title: String,
    pub post_id: u64,
    pub username: String,
    pub cover_url: String,
}