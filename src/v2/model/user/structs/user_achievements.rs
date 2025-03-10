// src/structs/user_achievements.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAchievements {
    pub achieved_at: String,
    pub achievement_id: u32,
}
