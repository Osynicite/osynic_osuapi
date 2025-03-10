use serde::{Deserialize, Serialize};

use super::failtimes::Failtimes;
use super::owner::Owner;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Convert {
    beatmapset_id: u32,
    difficulty_rating: f32,
    id: u32,
    mode: String,
    status: String,
    total_length: u32,
    user_id: u32,
    version: String,
    accuracy: f32,
    ar: f32,
    bpm: f32,
    convert: bool,
    count_circles: u32,
    count_sliders: u32,
    count_spinners: u32,
    cs: f32,
    deleted_at: Option<String>,
    drain: f32,
    hit_length: u32,
    is_scoreable: bool,
    last_updated: String,
    mode_int: u32,
    passcount: u32,
    playcount: u32,
    ranked: u32,
    url: String,
    checksum: String,
    failtimes: Failtimes,
    owners: Vec<Owner>,
}
