use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Language{
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

impl Language{
    pub fn to_string(&self) -> String {
        match self {
            Language::Other => "Other".to_string(),
            Language::English => "English".to_string(),
            Language::Japanese => "Japanese".to_string(),
            Language::Chinese => "Chinese".to_string(),
            Language::Instrumental => "Instrumental".to_string(),
            Language::Korean => "Korean".to_string(),
            Language::French => "French".to_string(),
            Language::German => "German".to_string(),
            Language::Swedish => "Swedish".to_string(),
            Language::Spanish => "Spanish".to_string(),
            Language::Italian => "Italian".to_string(),
            Language::Russian => "Russian".to_string(),
            Language::Polish => "Polish".to_string(),
            Language::Unspecified => "Unspecified".to_string(),
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










