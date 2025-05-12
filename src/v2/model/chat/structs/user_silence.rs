use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSilence {
    pub id: u32,
    pub user_id: u32,
}