use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd,
)]
pub enum Mode {
    #[default]
    #[serde(rename = "osu")]
    Osu = 0,
    #[serde(rename = "mania")]
    Mania = 1,
    #[serde(rename = "taiko")]
    Taiko = 2,
    #[serde(rename = "fruits")]
    Catch = 3,
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Mode::Osu => "Osu",
            Mode::Mania => "Mania",
            Mode::Taiko => "Taiko",
            Mode::Catch => "Catch",
        }
    }

    pub fn to_ruleset(&self) -> String {
        match self {
            Mode::Osu => "osu".to_string(),
            Mode::Mania => "mania".to_string(),
            Mode::Taiko => "taiko".to_string(),
            Mode::Catch => "fruits".to_string(),
        }
    }

    pub fn to_beatmapset_search(&self) -> String {
        match self {
            Mode::Osu => "0".to_string(),
            Mode::Mania => "1".to_string(),
            Mode::Taiko => "2".to_string(),
            Mode::Catch => "3".to_string(),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
