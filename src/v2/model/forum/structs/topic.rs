// Forum Topic
// Field 	Type
// created_at 	Timestamp
// deleted_at 	Timestamp?
// first_post_id 	integer
// forum_id 	integer
// id 	integer
// is_locked 	boolean
// last_post_id 	integer
// poll 	Poll?
// post_count 	integer
// title 	string
// type 	normal | sticky | announcement
// updated_at 	Timestamp
// user_id 	integer
// Poll
// Field 	Type
// allow_vote_change 	boolean
// ended_at 	Timestamp?
// hide_incomplete_results 	boolean
// last_vote_at 	Timestamp?
// max_votes 	integer
// options 	PollOption[]
// started_at 	Timestamp
// title.bbcode 	string
// title.html 	string
// total_vote_count 	integer
// PollOption
// Field 	Type 	Notes
// id 	integer 	Unique only per-topic.
// text.bbcode 	string
// text.html 	string
// vote_count 	integer? 	Not present if the poll is incomplete and results are hidden.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicListing {
    pub topics: Vec<ForumTopic>,
    pub cursor_string: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopic {
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub first_post_id: u64,
    pub forum_id: u64,
    pub id: u64,
    pub is_locked: bool,
    pub last_post_id: u64,
    pub poll: Option<Poll>,
    pub post_count: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub topic_type: String,
    pub updated_at: String,
    pub user_id: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Poll {
    pub allow_vote_change: bool,
    pub ended_at: Option<String>,
    pub hide_incomplete_results: bool,
    pub last_vote_at: Option<String>,
    pub max_votes: u64,
    pub options: Vec<PollOption>,
    pub started_at: String,
    pub title: Body,
    pub total_vote_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    pub id: u64,
    pub text: Body,
    pub vote_count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub bbcode: String,
    pub html: String,
}
