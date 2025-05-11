use serde::{Deserialize, Serialize};

use crate::v2::model::event::structs::event::Event;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventsResponse {
    pub events: Vec<Event>,
    pub cursor: Cursor,
    pub cursor_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub event_id: u64,
}