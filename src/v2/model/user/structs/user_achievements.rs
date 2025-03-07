// src/structs/user_achievements.rs

use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct UserAchievements {
    
    pub achieved_at: String,
    pub achievement_id: u32,
}