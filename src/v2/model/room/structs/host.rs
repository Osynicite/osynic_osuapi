// "host": {
// 		"avatar_url": "https:\/\/a.ppy.sh\/15403214?1736477293.jpeg",
// 		"country_code": "US",
// 		"default_group": "default",
// 		"id": 15403214,
// 		"is_active": true,
// 		"is_bot": false,
// 		"is_deleted": false,
// 		"is_online": true,
// 		"is_supporter": false,
// 		"last_visit": "2025-05-12T06:10:24+00:00",
// 		"pm_friends_only": false,
// 		"profile_colour": null,
// 		"username": "Draconify",
// 		"country": {
// 			"code": "US",
// 			"name": "United States"
// 		}
// 	},

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
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
    pub country: Option<Country>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}
