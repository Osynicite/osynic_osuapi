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
    pub beatmaps: Option<Vec<Beatmap>>,

    // ----Extended Info----
    pub pack_tags: Option<Vec<String>>,
    pub converts: Option<Vec<Convert>>,
    pub current_nominations: Option<Vec<CurrentNomination>>,
    pub description: Option<Description>,
    pub genre: Option<Genre>,
    pub language: Option<Language>,
    pub ratings: Option<Vec<u32>>,
    pub recent_favourites: Option<Vec<User>>,
    pub related_users: Option<Vec<User>>,
    pub related_tags: Option<Vec<String>>,
    pub user: Option<User>,
}
