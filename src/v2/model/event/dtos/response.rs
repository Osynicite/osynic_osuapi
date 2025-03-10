use serde::{Deserialize, Serialize};

use crate::v2::model::event::structs::event::Event;
use crate::v2::model::search::structs::cursor::Cursor;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventsResponse {
    pub events: Vec<Event>,
    pub cursor: Cursor,
    pub cursor_string: String,
}
