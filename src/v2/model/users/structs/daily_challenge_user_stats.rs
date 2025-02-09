// src/structs/daily_challenge_user_stats.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyChallengeUserStats {
    pub daily_streak_best: u64,
    pub daily_streak_current: u64,
    
    pub last_update: String,
    
    pub last_weekly_streak: String,
    pub playcount: u64,
    pub top_10p_placements: u64,
    pub top_50p_placements: u64,
    pub user_id: u64,
    pub weekly_streak_best: u64,
    pub weekly_streak_current: u64,
}

impl Default for DailyChallengeUserStats {
    fn default() -> Self {
        DailyChallengeUserStats {
            daily_streak_best: 0,
            daily_streak_current: 0,
            last_update: "".to_string(),
            last_weekly_streak: "".to_string(),
            playcount: 0,
            top_10p_placements: 0,
            top_50p_placements: 0,
            user_id: 0,
            weekly_streak_best: 0,
            weekly_streak_current: 0,
        }
    }
}