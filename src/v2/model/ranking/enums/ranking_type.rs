// RankingType

// Available ranking types:
// Name 	Description
// charts 	Spotlight
// country 	Country
// performance 	Performance
// score 	Score

use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd,
)]
pub enum RankingType {
    #[default]
    #[serde(rename = "charts")]
    Charts = 0,
    #[serde(rename = "country")]
    Country = 1,
    #[serde(rename = "performance")]
    Performance = 2,
    #[serde(rename = "score")]
    Score = 3,
}

impl RankingType {
    pub fn as_str(&self) -> &str {
        match self {
            RankingType::Charts => "Charts",
            RankingType::Country => "Country",
            RankingType::Performance => "Performance",
            RankingType::Score => "Score",
        }
    }

    pub fn to_ranking_type(&self) -> String {
        match self {
            RankingType::Charts => "charts".to_string(),
            RankingType::Country => "country".to_string(),
            RankingType::Performance => "performance".to_string(),
            RankingType::Score => "score".to_string(),
        }
    }
}

impl std::fmt::Display for RankingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
