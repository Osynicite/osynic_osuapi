use serde::{Deserialize, Serialize};

use super::non_legacy::acronym_mod::AcronymMod;
use super::non_legacy::statistics::Statistics;
use crate::v2::model::score::structs::current_user_attributes::CurrentUserAttributes;
use crate::v2::model::score::structs::user::User;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonLegacyScore {
    pub classic_total_score: u64,
    pub preserve: bool,
    pub processed: bool,
    pub ranked: bool,
    pub maximum_statistics: Statistics,
    pub mods: Vec<AcronymMod>,
    pub statistics: Statistics,
    pub beatmap_id: u64,
    pub best_id: Option<u64>,
    pub id: u64,
    pub rank: String,
    #[serde(rename = "type")]
    pub ranked_type: String,
    pub user_id: u64,
    pub accuracy: f32,
    pub build_id: Option<u64>,
    pub ended_at: String,
    pub has_replay: bool,
    pub is_perfect_combo: bool,
    pub legacy_perfect: bool,
    pub legacy_score_id: Option<u64>,
    pub legacy_total_score: u64,
    pub max_combo: u32,
    pub passed: bool,
    pub pp: f32,
    pub ruleset_id: u32,
    pub started_at: Option<String>,
    pub total_score: u64,
    pub replay: bool,
    pub current_user_attributes: Option<CurrentUserAttributes>,
    pub user: Option<User>,
}
