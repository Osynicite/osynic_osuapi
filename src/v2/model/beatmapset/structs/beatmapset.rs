use serde::{Serialize, Deserialize};



use crate::v2::model::beatmapset::enums::status::Status;
use crate::v2::model::beatmapset::structs::covers::Covers;
use crate::v2::model::beatmapset::structs::nominations_summary::NominationsSummary;
use crate::v2::model::beatmapset::structs::availability::Availability;
use crate::v2::model::beatmap::structs::beatmap::Beatmap;


#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Beatmapset {
    pub artist: String,
    pub artist_unicode: String,
    pub covers: Covers,
    pub creator: String,
    pub favourite_count: u32,
    pub hype: Option<u32>,
    pub id: u32,
    pub nsfw: bool,
    pub offset: i32,
    pub play_count: u32,
    pub preview_url: String,
    pub source: String,
    pub spotlight: bool,
    // 按照字符串解析
    // pub status: String,
    pub status: Status,
    pub title: String,
    pub title_unicode: String,
    pub track_id: Option<u32>,
    pub user_id: u32,
    pub video: bool,
    pub bpm: f32,
    pub can_be_hyped: bool,
    pub deleted_at: Option<String>,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub is_scoreable: bool,
    pub last_updated: String,
    pub legacy_thread_url: String,
    pub nominations_summary: NominationsSummary,
    pub ranked: u32,
    pub ranked_date: String,
    pub storyboard: bool,
    pub submitted_date: String,
    pub tags: String,
    pub availability: Availability,
    pub beatmaps: Vec<Beatmap>,
    pub pack_tags: Vec<String>,
}
