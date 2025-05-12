use serde::{Deserialize, Serialize};

use crate::v2::model::matches::structs::matches::Match;
// use crate::v2::model::user::structs::user::User;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMatchesListingResponse {
    pub matches: Vec<Match>,
    pub params: Params,
    pub cursor: Cursor,
    pub cursor_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub match_id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub limit: u32,
    pub sort: String,
}

// {
//     "matches": [
//         {
//             "id": 114428685,
//             "start_time": "2024-06-25T00:55:30+00:00",
//             "end_time": null,
//             "name": "peppy's game"
//         },
//         // ...
//     ],
//     "params": {
//         "limit": 50,
//         "sort": "id_desc"
//     },
//     "cursor": {
//         "match_id": 114428685
//     },
//     "cursor_string": "eyJtYXRjaF9pZCI6MTE0NDI4Njg1fQ"
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMatchResponse {
    #[serde(rename = "match")]
    pub matchh: Match,
    pub events: Vec<Event>,
    pub users: Vec<User>,
    pub first_event_id: u64,
    pub latest_event_id: u64,
    pub current_game_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: u64,
    pub detail: Detail,
    pub timestamp: String,
    pub user_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detail {
    #[serde(rename = "type")]
    pub event_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
    pub id: u64,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

// {
// 		"avatar_url": "https:\/\/a.ppy.sh\/11690784?1739479212.jpeg",
// 		"country_code": "ZA",
// 		"default_group": "default",
// 		"id": 11690784,
// 		"is_active": true,
// 		"is_bot": false,
// 		"is_deleted": false,
// 		"is_online": false,
// 		"is_supporter": true,
// 		"last_visit": null,
// 		"pm_friends_only": false,
// 		"profile_colour": null,
// 		"username": "DragonMage",
// 		"country": {
// 			"code": "ZA",
// 			"name": "South Africa"
// 		}
// {
//     "match": {
//         "id": 16155689,
//         "start_time": "2015-05-16T09:44:51+00:00",
//         "end_time": "2015-05-16T10:55:08+00:00",
//         "name": "CWC 2015: (Australia) vs (Poland)"
//     },
//     "events": [
//         {
//             "id": 484385927,
//             "detail": {
//                 "type": "match-created"
//             },
//             "timestamp": "2015-05-16T09:44:51+00:00",
//             "user_id": null
//         },
//         // ...
//     ],
//     "users": [],
//     "first_event_id": 484385927,
//     "latest_event_id": 484410607,
//     "current_game_id": null
// }
