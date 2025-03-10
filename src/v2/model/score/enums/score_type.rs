use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum ScoreType {
    #[serde(rename = "best")]
    Best,
    #[serde(rename = "first")]
    First,
    #[serde(rename = "recent")]
    Recent,
}

impl ScoreType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScoreType::Best => "Best",
            ScoreType::First => "First",
            ScoreType::Recent => "Recent",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            ScoreType::Best => "Best".to_string(),
            ScoreType::First => "First".to_string(),
            ScoreType::Recent => "Recent".to_string(),
        }
    }
    pub fn to_param(&self) -> String {
        match self {
            ScoreType::Best => "best".to_string(),
            ScoreType::First => "first".to_string(),
            ScoreType::Recent => "recent".to_string(),
        }
    }
}
