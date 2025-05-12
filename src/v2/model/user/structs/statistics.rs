// src/structs/statistics.rs

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub count_100: u32,
    pub count_300: u32,
    pub count_50: u32,
    pub count_miss: u32,
    pub country_rank: Option<u32>,
    pub level: Level,
    pub global_rank: Option<u32>,
    pub global_rank_exp: Option<u32>,
    pub pp: f64,
    pub pp_exp: Option<f64>,
    pub ranked_score: u64,
    pub hit_accuracy: f64,
    pub play_count: u32,
    pub play_time: u32,
    pub total_score: u64,
    pub total_hits: u64,
    pub maximum_combo: u32,
    pub replays_watched_by_others: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_change_since_30_days: Option<i32>,
    pub is_ranked: bool,
    pub grade_counts: GradeCounts,
    pub rank: Option<Rank>,
    pub variants: Option<Vec<Variant>>,
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub current: u32,
    pub progress: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GradeCounts {
    pub ss: u32,
    pub ssh: u32,
    pub s: u32,
    pub sh: i32,
    pub a: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rank {
    pub global: Option<u32>,
    pub country: u32,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub country_rank: u32,
    pub global_rank: u32,
    pub mode: String,
    pub pp: f64,
    pub variant: String,
}


//  "user": {
//       "avatar_url": "https://a.ppy.sh/2?1519081077.png",
//       "country": {
//           "code": "AU",
//           "name": "Australia"
//       },
//       "country_code": "AU",
//       "cover": {
//           "custom_url": null,
//           "id": "3",
//           "url": "https://assets.ppy.sh/user-profile-covers/2/baba245ef60834b769694178f8f6d4f6166c5188c740de084656ad2b80f1eea7.jpeg"
//       },
//       "default_group": "ppy",
//       "id": 2,
//       "is_active": false,
//       "is_bot": false,
//       "is_online": false,
//       "is_supporter": true,
//       "last_visit": "2019-02-22T11:07:10+00:00",
//       "pm_friends_only": false,
//       "profile_colour": "#3366FF",
//       "username": "peppy"

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country: Option<Country>,
    pub country_code: String,
    pub cover: Option<Cover>,
    pub default_group: String,
    pub id: u32,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<String>,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cover {
    pub custom_url: Option<String>,
    pub id: Option<String>,
    pub url: String,
}