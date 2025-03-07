use serde::{Serialize, Deserialize};

use crate::v2::model::score::enums::mode::Mode;
use crate::v2::model::score::enums::rank::Rank;

use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::score::structs::current_user_attributes::CurrentUserAttributes;
use crate::v2::model::score::structs::statistics::Statistics;
use crate::v2::model::score::structs::user::User;

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Score {
    pub accuracy: f64,
    pub best_id: Option<u32>,
    pub created_at: String,
    pub id: u32,
    pub max_combo: u32,
    pub mode: Mode,
    pub mode_int: u32,
    pub mods: Vec<String>,
    pub passed: bool,
    pub perfect: bool,
    pub pp: f64,
    pub rank: Rank,
    pub replay: bool,
    pub score: u32,
    pub statistics: Statistics,
    #[serde(rename = "type")]
    pub ranking_type: String,
    pub user_id: u32,
    pub current_user_attributes: CurrentUserAttributes,
    pub beatmap: Option<Beatmap>,
    pub user: Option<User>,
}
