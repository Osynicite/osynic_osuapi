// src/structs/daily_challenge_user_stats.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyChallengeUserStats {
    pub daily_streak_best: u32,
    pub daily_streak_current: u32,

    pub last_update: String,

    pub last_weekly_streak: String,
    pub playcount: u32,
    pub top_10p_placements: u32,
    pub top_50p_placements: u32,
    pub user_id: u32,
    pub weekly_streak_best: u32,
    pub weekly_streak_current: u32,
}
