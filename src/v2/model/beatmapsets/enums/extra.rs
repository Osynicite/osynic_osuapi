use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Extra {
    Video = 0,
    Storyboard = 1
}

impl Extra {
    pub fn to_string(&self) -> String {
        match self {
            Extra::Video => "Video".to_string(),
            Extra::Storyboard => "Storyboard".to_string()
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Extra::Video => "video".to_string(),
            Extra::Storyboard => "storyboard".to_string()
        }
    }
}