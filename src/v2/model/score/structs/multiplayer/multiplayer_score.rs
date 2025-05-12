// {
// 		"playlist_item_id": 16008432,
// 		"room_id": 1354141,
// 		"solo_score_id": 4830807461,
// 		"classic_total_score": 28586519,
// 		"preserve": true,
// 		"processed": true,
// 		"ranked": true,
// 		"maximum_statistics": {
// 			"great": 944,
// 			"ignore_hit": 626,
// 			"large_tick_hit": 172,
// 			"slider_tail_hit": 626
// 		},
// 		"mods": [{
// 			"acronym": "MR"
// 		}],
// 		"statistics": {
// 			"ok": 9,
// 			"great": 935,
// 			"ignore_hit": 626,
// 			"ignore_miss": 3,
// 			"large_tick_hit": 172,
// 			"slider_tail_hit": 623
// 		},
// 		"total_score_without_mods": 981535,
// 		"beatmap_id": 1383338,
// 		"best_id": null,
// 		"id": 4830807461,
// 		"rank": "S",
// 		"type": "solo_score",
// 		"user_id": 3216107,
// 		"accuracy": 0.994114,
// 		"build_id": 7980,
// 		"ended_at": "2025-05-12T04:10:33Z",
// 		"has_replay": true,
// 		"is_perfect_combo": false,
// 		"legacy_perfect": false,
// 		"legacy_score_id": null,
// 		"legacy_total_score": 0,
// 		"max_combo": 1739,
// 		"passed": true,
// 		"pp": null,
// 		"ruleset_id": 0,
// 		"started_at": "2025-05-12T04:04:35Z",
// 		"total_score": 981535,
// 		"replay": true,
// 		"current_user_attributes": {
// 			"pin": null
// 		},
// 		"user": {
// 			"avatar_url": "https:\/\/a.ppy.sh\/3216107?1728272508.jpeg",
// 			"country_code": "KR",
// 			"default_group": "gmt",
// 			"id": 3216107,
// 			"is_active": true,
// 			"is_bot": false,
// 			"is_deleted": false,
// 			"is_online": false,
// 			"is_supporter": true,
// 			"last_visit": null,
// 			"pm_friends_only": false,
// 			"profile_colour": "#99EB47",
// 			"username": "Civil oath",
// 			"country": {
// 				"code": "KR",
// 				"name": "South Korea"
// 			},
// 			"cover": {
// 				"custom_url": "https:\/\/assets.ppy.sh\/user-profile-covers\/3216107\/1c7062267e361442f8a0e908416c1e974a7290decc84ca7a97be94fa1ac58fd2.png",
// 				"url": "https:\/\/assets.ppy.sh\/user-profile-covers\/3216107\/1c7062267e361442f8a0e908416c1e974a7290decc84ca7a97be94fa1ac58fd2.png",
// 				"id": null
// 			}
// 		}
// 	}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiplayerScore {
    pub playlist_item_id: u32,
    pub room_id: u32,
    pub solo_score_id: u64,
    pub classic_total_score: u32,
    pub preserve: bool,
    pub processed: bool,
    pub ranked: bool,
    pub maximum_statistics: Statistics,
    pub mods: Vec<Mod>,
    pub statistics: Statistics,
    pub total_score_without_mods: u32,
    pub beatmap_id: u32,
    pub best_id: Option<u32>,
    pub id: u64,
    pub rank: String,
    #[serde(rename = "type")]
    pub score_type: String,
    pub user_id: u32,
    pub accuracy: f64,
    pub build_id: u32,
    pub ended_at: String,
    pub has_replay: bool,
    pub is_perfect_combo: bool,
    pub legacy_perfect: bool,
    pub legacy_score_id: Option<u32>,
    pub legacy_total_score: u32,
    pub max_combo: u32,
    pub passed: bool,
    pub pp: Option<f64>,
    pub ruleset_id: u32,
    pub started_at: String,
    pub total_score: u64,
    pub replay: bool,
    pub current_user_attributes: CurrentUserAttributes,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub ok: Option<u32>,
    pub great: Option<u32>,
    pub miss: Option<u32>,
    pub ignore_hit: Option<u32>,
    pub ignore_miss: Option<u32>,
    pub large_tick_hit: Option<u32>,
    pub slider_tail_hit: Option<u32>,
    pub small_tick_hit: Option<u32>,
    pub small_tick_miss: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mod {
    pub acronym: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserAttributes {
    pub pin: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
    pub id: u32,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<String>,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,
    pub country: Country,
    pub cover: Cover,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}

