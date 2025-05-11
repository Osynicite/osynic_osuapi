// src/structs/team.rs
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub flag_url: String,
    pub id: u64,
    pub name: String,
    pub short_name: String,
}

	// "team": {
	// 		"flag_url": "https:\/\/assets.ppy.sh\/teams\/flag\/1\/b46fb10dbfd8a35dc50e6c00296c0dc6172dffc3ed3d3a4b379277ba498399fe.png",
	// 		"id": 1,
	// 		"name": "mom?",
	// 		"short_name": "MOM"
	// 	}
