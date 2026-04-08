use serde::{Deserialize, Serialize};

use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::beatmapset::enums::status::Status;
use crate::v2::model::beatmapset::structs::availability::Availability;
use crate::v2::model::beatmapset::structs::covers::Covers;
use crate::v2::model::beatmapset::structs::nominations_summary::NominationsSummary;

use super::extended::convert::Convert;
use super::extended::current_nomination::CurrentNomination;
use super::extended::description::Description;
use super::extended::genre::Genre;
use super::extended::language::Language;
use super::extended::user::User;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub status: Status,
    pub title: String,
    pub title_unicode: String,
    pub track_id: Option<u32>,
    pub user_id: u32,
    pub video: bool,

    pub bpm: Option<f32>,
    pub can_be_hyped: Option<bool>,
    pub deleted_at: Option<String>,
    pub discussion_enabled: Option<bool>,
    pub discussion_locked: Option<bool>,
    pub is_scoreable: Option<bool>,
    pub last_updated: Option<String>,
    pub legacy_thread_url: Option<String>,
    pub nominations_summary: Option<NominationsSummary>,
    pub ranked: Option<i32>,
    pub ranked_date: Option<String>,
    pub storyboard: Option<bool>,
    pub submitted_date: Option<String>,
    pub tags: Option<String>,
    pub availability: Option<Availability>,
    pub beatmaps: Option<Vec<Beatmap>>,

    // ----Extended Info----
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub converts: Option<Vec<Convert>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_nominations: Option<Vec<CurrentNomination>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<Genre>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_favourites: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_users: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
