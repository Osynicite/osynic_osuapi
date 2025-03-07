use crate::v2::model::{beatmapset::structs::beatmapset::Beatmapset, search::structs::cursor::Cursor};
use crate::v2::model::search::structs::search::Search;

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
