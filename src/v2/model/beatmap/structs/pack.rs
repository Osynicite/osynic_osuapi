// BeatmapPack

// Represent a beatmap pack.
// Field 	Type 	Description
// author 	string
// date 	Timestamp
// name 	string
// no_diff_reduction 	boolean 	Whether difficulty reduction mods may be used to clear the pack.
// ruleset_id 	integer
// tag 	string 	The tag of the beatmap pack. Starts with a character representing the type (See the Tag column of BeatmapPackType) followed by an integer.
// url 	string 	The download url of the beatmap pack.
// Optional Attributes
// Field 	Type 	Description
// beatmapsets 	Beatmapset[]
// user_completion_data.beatmapset_ids 	integer[] 	IDs of beatmapsets completed by the user (according to the requirements of the pack)
// user_completion_data.completed 	boolean 	Whether all beatmapsets are completed or not

use serde::{Deserialize, Serialize};

use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapPack {
    /// The author of the beatmap pack.
    pub author: String,
    /// The date of the beatmap pack.
    pub date: String,
    /// The name of the beatmap pack.
    pub name: String,
    /// Whether difficulty reduction mods may be used to clear the pack.
    pub no_diff_reduction: bool,
    /// The ruleset ID of the beatmap pack.
    pub ruleset_id: Option<u32>,
    /// The tag of the beatmap pack. Starts with a character representing the type (See the Tag column of BeatmapPackType) followed by an integer.
    pub tag: String,
    /// The download URL of the beatmap pack.
    pub url: String,
    /// The beatmapsets of the beatmap pack.
    pub beatmapsets: Option<Vec<Beatmapset>>,
    /// The user completion data of the beatmap pack.
    pub user_completion_data: Option<UserCompletionData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCompletionData {
    /// IDs of beatmapsets completed by the user (according to the requirements of the pack)
    pub beatmapset_ids: Option<Vec<i32>>,
    /// Whether all beatmapsets are completed or not
    pub completed: Option<bool>,
}