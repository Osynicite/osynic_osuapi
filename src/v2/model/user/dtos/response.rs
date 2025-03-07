use serde::{Serialize, Deserialize};
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::beatmap::structs::beatmap_playcount::BeatmapPlaycount;


#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct GetUserBeatmapsResponse{
    pub beatmapsets: Option<Vec<Beatmapset>>,
    pub beatmap_playcounts: Option<Vec<BeatmapPlaycount>>,
    pub error: Option<String>,
}
