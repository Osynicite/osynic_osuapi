// Comments and related data.
// Field 	Type 	Description
// commentable_meta 	CommentableMeta[] 	ID of the object the comment is attached to
// comments 	Comment[] 	Array of comments ordered according to sort;
// cursor 	Cursor
// has_more 	boolean 	If there are more comments or replies available
// has_more_id 	integer?
// included_comments 	Comment[] 	Related comments; e.g. parent comments and nested replies
// pinned_comments 	Comment[]? 	Pinned comments
// sort 	string 	one of the CommentSort types
// top_level_count 	integer? 	Number of comments at the top level. Not returned for replies.
// total 	integer? 	Total number of comments. Not retuned for replies.
// user_follow 	boolean 	is the current user watching the comment thread?
// user_votes 	integer[] 	IDs of the comments in the bundle the current user has upvoted
// users 	User[] 	array of users related to the comments

use serde::{Deserialize, Serialize};

use crate::v2::model::comment::structs::comment::Comment;
use crate::v2::model::comment::structs::meta::CommentableMeta;
use crate::v2::model::comment::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentBundle {
    pub commentable_meta: Option<CommentableMeta>,
    pub comments: Vec<Comment>,
    pub cursor: Option<String>,
    pub has_more: bool,
    pub has_more_id: Option<i64>,
    pub included_comments: Vec<Comment>,
    pub pinned_comments: Option<Vec<Comment>>,
    pub sort: String,
    pub top_level_count: Option<i64>,
    pub total: Option<i64>,
    pub user_follow: bool,
    pub user_votes: Vec<i64>,
    pub users: Vec<User>,
}
