// MultiplayerScores

// An object which contains scores and related data for fetching next page of the result.
// Field 	Type 	Description
// cursor_string 	CursorString 	To be used to fetch the next page.
// params 	object 	Parameters used for score listing.
// scores 	Score[] 	
// total 	integer? 	Index only. Total scores of the specified playlist item.
// user_score 	Score? 	Index only. Score of the accessing user if exists.

use serde::{Deserialize, Serialize};

use crate::v2::model::score::structs::score::Score;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiplayerScores {
    pub cursor_string: Option<String>,
    pub params: Params,
    pub scores: Vec<Score>,
    pub total: Option<i64>,
    pub user_score: Option<Score>,
}

// wried docs...
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub idk: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiplayerScoresAround {
    pub higher: MultiplayerScores,
    pub lower: MultiplayerScores
}