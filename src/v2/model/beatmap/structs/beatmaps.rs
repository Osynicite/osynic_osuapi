use serde::{Serialize, Deserialize};

use super::beatmap::Beatmap;

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Beatmaps {
    pub beatmaps: Vec<Beatmap>
}