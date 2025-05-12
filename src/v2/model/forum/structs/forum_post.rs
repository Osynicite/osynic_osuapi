// Forum Post
// Field 	Type 	Description
// created_at 	Timestamp 	
// deleted_at 	Timestamp? 	
// edited_at 	Timestamp? 	
// edited_by_id 	integer? 	
// forum_id 	integer 	
// id 	integer 	
// topic_id 	integer 	
// user_id 	integer 	

// Following fields are optional.
// Field 	Type 	Description
// body.html 	string 	Post content in HTML format.
// body.raw 	string 	Post content in BBCode format.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumPost {
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub edited_at: Option<String>,
    pub edited_by_id: Option<u64>,
    pub forum_id: u64,
    pub id: u64,
    pub topic_id: u64,
    pub user_id: u64,
    pub body: Option<Body>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub html: String,
    pub raw: String,
}