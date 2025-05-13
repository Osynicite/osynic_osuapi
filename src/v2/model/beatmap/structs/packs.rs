
use serde::{Deserialize, Serialize};

use super::pack::BeatmapPack;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapPacks {
    /// The beatmap packs of the beatmap packs.
    pub beatmap_packs: Vec<BeatmapPack>,
}