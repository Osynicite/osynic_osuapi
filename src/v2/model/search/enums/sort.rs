use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Sort {
    TitleDesc = 0,
    TitleAsc = 1,
    ArtistDesc = 2,
    ArtistAsc = 3,
    DifficultyDesc = 4,
    DifficultyAsc = 5,
    RankedDesc = 6,
    RankedAsc = 7,
    RatingDesc = 8,
    RatingAsc = 9,
    PlaysDesc = 10,
    PlaysAsc = 11,
    FavouritesDesc = 12,
    FavouritesAsc = 13,
}

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Sort::TitleDesc => "TitleDesc",
            Sort::TitleAsc => "TitleAsc",
            Sort::ArtistDesc => "ArtistDesc",
            Sort::ArtistAsc => "ArtistAsc",
            Sort::DifficultyDesc => "DifficultyDesc",
            Sort::DifficultyAsc => "DifficultyAsc",
            Sort::RankedDesc => "RankedDesc",
            Sort::RankedAsc => "RankedAsc",
            Sort::RatingDesc => "RatingDesc",
            Sort::RatingAsc => "RatingAsc",
            Sort::PlaysDesc => "PlaysDesc",
            Sort::PlaysAsc => "PlaysAsc",
            Sort::FavouritesDesc => "FavouritesDesc",
            Sort::FavouritesAsc => "FavouritesAsc",
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Sort::TitleDesc => "title_desc".to_string(),
            Sort::TitleAsc => "title_asc".to_string(),
            Sort::ArtistDesc => "artist_desc".to_string(),
            Sort::ArtistAsc => "artist_asc".to_string(),
            Sort::DifficultyDesc => "difficulty_desc".to_string(),
            Sort::DifficultyAsc => "difficulty_asc".to_string(),
            Sort::RankedDesc => "ranked_desc".to_string(),
            Sort::RankedAsc => "ranked_asc".to_string(),
            Sort::RatingDesc => "rating_desc".to_string(),
            Sort::RatingAsc => "rating_asc".to_string(),
            Sort::PlaysDesc => "plays_desc".to_string(),
            Sort::PlaysAsc => "plays_asc".to_string(),
            Sort::FavouritesDesc => "favourites_desc".to_string(),
            Sort::FavouritesAsc => "favourites_asc".to_string(),
        }
    }
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
