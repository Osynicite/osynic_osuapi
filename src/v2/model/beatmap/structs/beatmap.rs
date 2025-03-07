use serde::{Serialize, Deserialize};

use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::beatmap::enums::status::Status;
use crate::v2::model::beatmap::enums::mode::Mode;

use super::extended::failtimes::Failtimes;
use super::extended::owner::Owner;

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Beatmap {
    pub beatmapset_id: u32,
    pub difficulty_rating: f32,
    pub id: u32,
    pub mode: Mode,
    pub status: Status,
    pub total_length: u32,
    pub user_id: u32,
    pub version: String,
    pub accuracy: f32,
    pub ar: f32,
    pub bpm: f32,
    pub convert: bool,
    pub count_circles: u32,
    pub count_sliders: u32,
    pub count_spinners: u32,
    pub cs: f32,
    pub deleted_at: Option<String>,
    pub drain: f32,
    pub hit_length: u32,
    pub is_scoreable: bool,
    pub last_updated: String,
    pub mode_int: u32,
    pub passcount: u32,
    pub playcount: u32,
    pub ranked: u32,
    pub url: String,
    pub checksum: String,
    
    
    // ----Extended Info----
    pub max_combo: Option<u32>,
    pub beatmapset: Option<Beatmapset>,
    pub current_user_tag_ids: Option<Vec<u32>>,
    pub failtimes: Option<Failtimes>,
    pub owners: Option<Vec<Owner>>,
    pub top_tag_ids: Option<Vec<u32>>,
}
