use serde::{Serialize, Deserialize};
use crate::v2::model::score::structs::non_legacy_score::NonLegacyScore;
use crate::v2::model::search::structs::cursor::Cursor;

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct GetScoresResponse{
    pub scores: Vec<NonLegacyScore>,
    pub cursor: Cursor,
    pub cursor_string: String,
}