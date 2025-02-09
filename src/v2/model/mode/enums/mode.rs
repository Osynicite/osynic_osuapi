use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub enum Mode{
    #[default]
    Osu,
    Mania,
    Taiko,
    Catch
}