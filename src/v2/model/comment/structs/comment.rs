// Represents a single comment.
// Field 	Type 	Description
// commentable_id 	integer 	ID of the object the comment is attached to
// commentable_type 	string 	type of object the comment is attached to
// created_at 	Timestamp 	ISO 8601 date
// deleted_at 	Timestamp? 	ISO 8601 date if the comment was deleted; null, otherwise
// edited_at 	Timestamp? 	ISO 8601 date if the comment was edited; null, otherwise
// edited_by_id 	integer? 	user id of the user that edited the post; null, otherwise
// id 	integer 	the ID of the comment
// legacy_name 	string? 	username displayed on legacy comments
// message 	string? 	markdown of the comment's content
// message_html 	string? 	html version of the comment's content
// parent_id 	integer? 	ID of the comment's parent
// pinned 	boolean 	Pin status of the comment
// replies_count 	integer 	Number of replies to the comment
// updated_at 	Timestamp 	ISO 8601 date
// user_id 	integer 	user ID of the poster
// votes_count 	integer 	Number of votes

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub commentable_id: i64,
    pub commentable_type: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub edited_at: Option<String>,
    pub edited_by_id: Option<i64>,
    pub id: i64,
    pub legacy_name: Option<String>,
    pub message: Option<String>,
    pub message_html: Option<String>,
    pub parent_id: Option<i64>,
    pub pinned: bool,
    pub replies_count: i64,
    pub updated_at: String,
    pub user_id: i64,
    pub votes_count: i64,
}