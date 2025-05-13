use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RankAchieved {
    XH = 0,
    X = 1,
    SH = 2,
    S = 3,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
}

impl RankAchieved {
    pub fn as_str(&self) -> &str {
        match self {
            RankAchieved::XH => "XH",
            RankAchieved::X => "X",
            RankAchieved::SH => "SH",
            RankAchieved::S => "S",
            RankAchieved::A => "A",
            RankAchieved::B => "B",
            RankAchieved::C => "C",
            RankAchieved::D => "D",
        }
    }

    pub fn to_beatmapset_search(&self) -> String {
        match self {
            RankAchieved::XH => "XH".to_string(),
            RankAchieved::X => "X".to_string(),
            RankAchieved::SH => "SH".to_string(),
            RankAchieved::S => "S".to_string(),
            RankAchieved::A => "A".to_string(),
            RankAchieved::B => "B".to_string(),
            RankAchieved::C => "C".to_string(),
            RankAchieved::D => "D".to_string(),
        }
    }
}

impl std::fmt::Display for RankAchieved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}