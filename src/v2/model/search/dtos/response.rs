use crate::v2::model::user::structs::user::User;
use crate::v2::model::wiki::WikiPage;
use crate::v2::model::search::structs::search::Search;
use crate::v2::model::search::structs::cursor::Cursor;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;

use serde::{Serialize, Deserialize};


#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct BeatmapsetsSearchResponse{
    pub beatmapsets: Vec<Beatmapset>,
    pub search: Search,
    pub recommended_difficulty: Option<String>,
    pub error: Option<String>,
    pub total: u32,
    pub cursor: Option<Cursor>,
    pub cursor_string: Option<String>
}

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct SearchResponse{
    pub user: Option<UserSearchResponse>,
    pub wiki_page: Option<WikiPageSearchResponse>,
}

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct UserSearchResponse{
    pub data: Vec<User>,
    pub total: u32,
}


#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct WikiPageSearchResponse{
    pub data: Vec<WikiPage>,
    pub total: u32,
}