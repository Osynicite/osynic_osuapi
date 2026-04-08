use serde::{Deserialize, Serialize};

use crate::v2::model::beatmap::enums::mode::Mode;
use crate::v2::model::beatmap::enums::status::Status;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;

use super::extended::failtimes::Failtimes;
use super::extended::owner::Owner;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub ranked: i32,
    pub url: String,
    pub checksum: String,

    // ----Extended Info----
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_combo: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beatmapset: Option<Beatmapset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_user_tag_ids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failtimes: Option<Failtimes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<Owner>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_tag_ids: Option<Vec<u32>>,
}
