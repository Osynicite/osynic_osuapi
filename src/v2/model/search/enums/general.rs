use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum General {
    #[default]
    Recommended = 0,
    Converts = 1,
    Follows = 2,
    Spotlights = 3,
    FeaturedArtists = 4,
}

impl General {
    pub fn as_str(&self) -> &str {
        match self {
            General::Recommended => "Recommended",
            General::Converts => "Converts",
            General::Follows => "Follows",
            General::Spotlights => "Spotlights",
            General::FeaturedArtists => "FeaturedArtists",
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            General::Recommended => "recommended".to_string(),
            General::Converts => "converts".to_string(),
            General::Follows => "follows".to_string(),
            General::Spotlights => "spotlights".to_string(),
            General::FeaturedArtists => "featured_artists".to_string(),
        }
    }
}

impl std::fmt::Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}