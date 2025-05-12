// Forum
// Field 	Type 	Notes
// id 	integer
// name 	string
// description 	string
// subforums 	Forum[]? 	Maximum 2 layers of subforums from the top-level Forum

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forum {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub subforums: Option<Vec<Forum>>,
}