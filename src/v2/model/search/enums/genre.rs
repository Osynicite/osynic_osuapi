use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd,
)]
pub enum Genre {
    #[default]
    Unspecified = 1,
    VideoGame = 2,
    Anime = 3,
    Rock = 4,
    Pop = 5,
    Other = 6,
    Novelty = 7,
    HipHop = 9,
    Electronic = 10,
    Metal = 11,
    Classical = 12,
    Folk = 13,
    Jazz = 14,
}

impl Genre {
    pub fn as_str(&self) -> &str {
        match self {
            Genre::Unspecified => "Unspecified",
            Genre::VideoGame => "VideoGame",
            Genre::Anime => "Anime",
            Genre::Rock => "Rock",
            Genre::Pop => "Pop",
            Genre::Other => "Other",
            Genre::Novelty => "Novelty",
            Genre::HipHop => "HipHop",
            Genre::Electronic => "Electronic",
            Genre::Metal => "Metal",
            Genre::Classical => "Classical",
            Genre::Folk => "Folk",
            Genre::Jazz => "Jazz",
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Genre::Unspecified => "1".to_string(),
            Genre::VideoGame => "2".to_string(),
            Genre::Anime => "3".to_string(),
            Genre::Rock => "4".to_string(),
            Genre::Pop => "5".to_string(),
            Genre::Other => "6".to_string(),
            Genre::Novelty => "7".to_string(),
            Genre::HipHop => "9".to_string(),
            Genre::Electronic => "10".to_string(),
            Genre::Metal => "11".to_string(),
            Genre::Classical => "12".to_string(),
            Genre::Folk => "13".to_string(),
            Genre::Jazz => "14".to_string(),
        }
    }
    pub fn from_id(id: u32) -> Genre {
        match id {
            1 => Genre::Unspecified,
            2 => Genre::VideoGame,
            3 => Genre::Anime,
            4 => Genre::Rock,
            5 => Genre::Pop,
            6 => Genre::Other,
            7 => Genre::Novelty,
            9 => Genre::HipHop,
            10 => Genre::Electronic,
            11 => Genre::Metal,
            12 => Genre::Classical,
            13 => Genre::Folk,
            14 => Genre::Jazz,
            _ => Genre::Unspecified,
        }
    }
}

impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}