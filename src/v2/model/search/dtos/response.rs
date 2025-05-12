use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::search::structs::cursor::Cursor;
use crate::v2::model::search::structs::search::Search;
use crate::v2::model::wiki::WikiPage;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapsetsSearchResponse {
    pub beatmapsets: Vec<Beatmapset>,
    pub search: Search,
    pub recommended_difficulty: Option<String>,
    pub error: Option<String>,
    pub total: u32,
    pub cursor: Option<Cursor>,
    pub cursor_string: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    pub user: Option<UserSearchResponse>,
    pub wiki_page: Option<WikiPageSearchResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSearchResponse {
    pub data: Vec<User>,
    pub total: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiPageSearchResponse {
    pub data: Vec<WikiPage>,
    pub total: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: Option<String>,
    pub id: u32,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<String>,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,
}
