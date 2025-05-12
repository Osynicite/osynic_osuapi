// "id": 16013549,
// 		"room_id": 1354457,
// 		"beatmap_id": 418631,
// 		"ruleset_id": 0,
// 		"allowed_mods": [],
// 		"required_mods": [],
// 		"freestyle": true,
// 		"expired": false,
// 		"owner_id": 15403214,
// 		"playlist_order": null,
// 		"played_at": null,

use serde::{Deserialize, Serialize};

use crate::v2::model::beatmapset::structs::covers::Covers;

use super::playlist_mod::PlaylistMod;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    pub id: u32,
    pub room_id: u32,
    pub beatmap_id: u32,
    pub ruleset_id: u32,
    pub allowed_mods: Vec<PlaylistMod>,
    pub required_mods: Vec<PlaylistMod>,
    pub freestyle: bool,
    pub expired: bool,
    pub owner_id: u32,
    pub playlist_order: Option<u32>,
    pub played_at: Option<String>,
    pub beatmap: Option<Beatmap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmap {
    pub beatmapset_id: u32,
    pub difficulty_rating: f64,
    pub id: u32,
    pub mode: String,
    pub status: String,
    pub total_length: u32,
    pub user_id: u32,
    pub version: String,
    pub beatmapset: BeatmapSet,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapSet {
    pub artist: String,
    pub artist_unicode: String,
    pub covers: Covers,
    pub creator: String,
    pub favourite_count: u32,
    pub hype: Option<u32>,
    pub id: u32,
    pub nsfw: bool,
    pub offset: u32,
    pub play_count: u32,
    pub preview_url: String,
    pub source: String,
    pub spotlight: bool,
    pub status: String,
    pub title: String,
    pub title_unicode: String,
    pub track_id: Option<u32>,
    pub user_id: u32,
    pub video: bool,
}

// "beatmap": {
// 			"beatmapset_id": 173234,
// 			"difficulty_rating": 4.23,
// 			"id": 418631,
// 			"mode": "osu",
// 			"status": "ranked",
// 			"total_length": 84,
// 			"user_id": 898306,
// 			"version": "Insane",
// 			"beatmapset": {
// 				"artist": "EGOIST",
// 				"artist_unicode": "EGOIST",
// 				"covers": {
// 					"cover": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/cover.jpg?1650622117",
// 					"cover@2x": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/cover@2x.jpg?1650622117",
// 					"card": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/card.jpg?1650622117",
// 					"card@2x": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/card@2x.jpg?1650622117",
// 					"list": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/list.jpg?1650622117",
// 					"list@2x": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/list@2x.jpg?1650622117",
// 					"slimcover": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/slimcover.jpg?1650622117",
// 					"slimcover@2x": "https:\/\/assets.ppy.sh\/beatmaps\/173234\/covers\/slimcover@2x.jpg?1650622117"
// 				},
// 				"creator": "Yauxo",
// 				"favourite_count": 2118,
// 				"hype": null,
// 				"id": 173234,
// 				"nsfw": false,
// 				"offset": 0,
// 				"play_count": 6354030,
// 				"preview_url": "\/\/b.ppy.sh\/preview\/173234.mp3",
// 				"source": "Guilty Crown",
// 				"spotlight": false,
// 				"status": "ranked",
// 				"title": "The Everlasting Guilty Crown -TV Edit-",
// 				"title_unicode": "The Everlasting Guilty Crown -TV Edit-",
// 				"track_id": null,
// 				"user_id": 898306,
// 				"video": true
// 			}
// 		}

// "playlist_item_stats": {
// 	"count_active": 43,
// 	"count_total": 43,
// 	"ruleset_ids": [0]
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistItemStats {
    pub count_active: u32,
    pub count_total: u32,
    pub ruleset_ids: Vec<u32>,
}