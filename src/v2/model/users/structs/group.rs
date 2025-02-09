use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    colour: String,
    has_listing: bool,
    has_playmodes: bool,
    id: u32,
    identifier: String,
    is_probationary: bool,
    name: String,
    short_name: String,
    playmodes: Option<Vec<String>>, // 使用 Option 来表示可能为 null 的字段
}