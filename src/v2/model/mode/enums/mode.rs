use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug,Default, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Mode{
    #[default]
    #[serde(rename = "osu")]
    Osu = 0,
    #[serde(rename = "mania")]
    Mania = 1,
    #[serde(rename = "taiko")]
    Taiko = 2,
    #[serde(rename = "fruits")]
    Catch = 3
}

impl Mode{
    pub fn to_string(&self) -> String{
        match self{
            Mode::Osu => "Osu".to_string(),
            Mode::Mania => "Mania".to_string(),
            Mode::Taiko => "Taiko".to_string(),
            Mode::Catch => "Catch".to_string()
        }
    }

    pub fn to_beatmapset_search(&self) -> String{
        match self{
            Mode::Osu => "0".to_string(),
            Mode::Mania => "1".to_string(),
            Mode::Taiko => "2".to_string(),
            Mode::Catch => "3".to_string()
        }
    }
}