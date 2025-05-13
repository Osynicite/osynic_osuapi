use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Extra {
    Video = 0,
    Storyboard = 1,
}

impl Extra {
    pub fn as_str(&self) -> &str {
        match self {
            Extra::Video => "Video",
            Extra::Storyboard => "Storyboard",
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Extra::Video => "video".to_string(),
            Extra::Storyboard => "storyboard".to_string(),
        }
    }
}

impl std::fmt::Display for Extra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}