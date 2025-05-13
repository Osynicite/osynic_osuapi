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
    pub fn to_param(&self) -> String {
        match self {
            ScoreType::Best => "best".to_string(),
            ScoreType::First => "first".to_string(),
            ScoreType::Recent => "recent".to_string(),
        }
    }
}

impl std::fmt::Display for ScoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
