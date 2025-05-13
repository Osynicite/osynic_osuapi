use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Language {
    Other = 1,
    English = 2,
    Japanese = 3,
    Chinese = 4,
    Instrumental = 5,
    Korean = 6,
    French = 7,
    German = 8,
    Swedish = 9,
    Spanish = 10,
    Italian = 11,
    Russian = 12,
    Polish = 13,
    Unspecified = 14,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Language::Other => "Other",
            Language::English => "English",
            Language::Japanese => "Japanese",
            Language::Chinese => "Chinese",
            Language::Instrumental => "Instrumental",
            Language::Korean => "Korean",
            Language::French => "French",
            Language::German => "German",
            Language::Swedish => "Swedish",
            Language::Spanish => "Spanish",
            Language::Italian => "Italian",
            Language::Russian => "Russian",
            Language::Polish => "Polish",
            Language::Unspecified => "Unspecified",
        }
    }
    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Language::Other => "1".to_string(),
            Language::English => "2".to_string(),
            Language::Japanese => "3".to_string(),
            Language::Chinese => "4".to_string(),
            Language::Instrumental => "5".to_string(),
            Language::Korean => "6".to_string(),
            Language::French => "7".to_string(),
            Language::German => "8".to_string(),
            Language::Swedish => "9".to_string(),
            Language::Spanish => "10".to_string(),
            Language::Italian => "11".to_string(),
            Language::Russian => "12".to_string(),
            Language::Polish => "13".to_string(),
            Language::Unspecified => "14".to_string(),
        }
    }
    pub fn from_id(id: u32) -> Language {
        match id {
            1 => Language::Other,
            2 => Language::English,
            3 => Language::Japanese,
            4 => Language::Chinese,
            5 => Language::Instrumental,
            6 => Language::Korean,
            7 => Language::French,
            8 => Language::German,
            9 => Language::Swedish,
            10 => Language::Spanish,
            11 => Language::Italian,
            12 => Language::Russian,
            13 => Language::Polish,
            14 => Language::Unspecified,
            _ => Language::Unspecified,
        }
    }
}


impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}