use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq, Serialize, Deserialize)] 
pub enum Mode{
    #[default]
    Osu = 0,
    Mania = 1,
    Taiko = 2,
    Catch = 3
}

impl Mode{
    pub fn to_string(&self) -> String{
        match self{
            Mode::Osu => "osu".to_string(),
            Mode::Mania => "mania".to_string(),
            Mode::Taiko => "taiko".to_string(),
            Mode::Catch => "catch".to_string()
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