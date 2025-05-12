// Field 	Type 	Description
// current_user_attributes 	CurrentUserAttributes 	
// id 	integer 	the ID of the object
// owner_id 	integer? 	User ID which owns the object
// owner_title 	string? 	Object owner type, used for display (MAPPER for beatmapset)
// title 	string 	display title
// type 	string 	the type of the object
// url 	string 	url of the objec

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentableMeta{
    pub current_user_attributes: Option<CurrentUserAttributes>,
    pub id: i64,
    pub owner_id: Option<i64>,
    pub owner_title: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub url: String,
}

// CurrentUserAttributes

// An object listing various related permissions and states for the current user, related to the object it is attached to.
// BeatmapsetDiscussionPermissions

// TODO: needs a better name.
// Name 	Description
// can_destroy 	Can delete the discussion.
// can_reopen 	Can reopen the discussion.
// can_moderate_kudosu 	Can allow or deny kudosu.
// can_resolve 	Can resolve the discussion.
// vote_score 	Current vote given to the discussion.
// ChatChannelUserAttributes
// Name 	Type 	Description
// can_message 	boolean 	Can send messages to this channel.
// can_message_error 	string? 	Reason messages cannot be sent to this channel
// last_read_id 	integer 	message_id of last message read.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserAttributes{
    pub can_new_comment_reason: Option<String>,
    pub can_destroy: bool,
    pub can_reopen: bool,
    pub can_moderate_kudosu: bool,
    pub can_resolve: bool,
    pub vote_score: i64,
    pub can_message: bool,
    pub can_message_error: Option<String>,
    pub last_read_id: i64,
}