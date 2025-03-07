// src/structs/user_achievements.rs

use serde::{Serialize, Deserialize};

// use crate::v2::model::users::enums::achievement::Achievement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAchievements {
    
    pub achieved_at: String,
    pub achievement_id: u32,
}