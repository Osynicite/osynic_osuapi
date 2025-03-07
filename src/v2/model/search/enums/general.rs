use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum General{
    #[default]
    Recommended = 0,
    Converts = 1,
    Follows = 2,
    Spotlights = 3,
    FeaturedArtists = 4
}

impl General{
    pub fn to_string(&self) -> String{
        match self{
            General::Recommended => "Recommended".to_string(),
            General::Converts => "Converts".to_string(),
            General::Follows => "Follows".to_string(),
            General::Spotlights => "Spotlights".to_string(),
            General::FeaturedArtists => "FeaturedArtists".to_string()
        }
    }
    pub fn to_beatmapset_search(&self) -> String{
        match self{
            General::Recommended => "recommended".to_string(),
            General::Converts => "converts".to_string(),
            General::Follows => "follows".to_string(),
            General::Spotlights => "spotlights".to_string(),
            General::FeaturedArtists => "featured_artists".to_string()
        }
    }
}