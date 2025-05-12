// {
// 			"acronym": "EZ",
// 			"settings": {}
// 		},

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistMod {
    pub acronym: String,
    pub settings: serde_json::Value,
}
