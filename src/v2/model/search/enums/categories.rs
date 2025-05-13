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
    pub fn as_str(&self) -> &str {
        match self {
            Categories::Any => "Any",
            Categories::Ranked => "Ranked",
            Categories::Qualified => "Qualified",
            Categories::Loved => "Loved",
            Categories::Favourites => "Favourites",
            Categories::Pending => "Pending",
            Categories::WIP => "WIP",
            Categories::Graveyard => "Graveyard",
            Categories::Mine => "Mine",
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

impl std::fmt::Display for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}