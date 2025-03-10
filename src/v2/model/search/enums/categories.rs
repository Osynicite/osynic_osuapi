use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Categories {
    Any = 0,
    Ranked = 1,
    Qualified = 2,
    Loved = 3,
    Favourites = 4,
    Pending = 5,
    WIP = 6,
    Graveyard = 7,
    Mine = 8,
}

impl Categories {
    pub fn to_string(&self) -> String {
        match self {
            Categories::Any => "Any".to_string(),
            Categories::Ranked => "Ranked".to_string(),
            Categories::Qualified => "Qualified".to_string(),
            Categories::Loved => "Loved".to_string(),
            Categories::Favourites => "Favourites".to_string(),
            Categories::Pending => "Pending".to_string(),
            Categories::WIP => "WIP".to_string(),
            Categories::Graveyard => "Graveyard".to_string(),
            Categories::Mine => "Mine".to_string(),
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Categories::Any => "any".to_string(),
            Categories::Ranked => "ranked".to_string(),
            Categories::Qualified => "qualified".to_string(),
            Categories::Loved => "loved".to_string(),
            Categories::Favourites => "favourites".to_string(),
            Categories::Pending => "pending".to_string(),
            Categories::WIP => "wip".to_string(),
            Categories::Graveyard => "graveyard".to_string(),
            Categories::Mine => "mine".to_string(),
        }
    }
}
