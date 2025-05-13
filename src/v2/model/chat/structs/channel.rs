// ChatChannel

// {
//   "channel_id": 1337,
//   "current_user_attributes": {
//     "can_message": true,
//     "can_message_error": null,
//     "last_read_id": 9150005005,
//   },
//   "name": "test channel",
//   "description": "wheeeee",
//   "icon": "/images/layout/avatar-guest@2x.png",
//   "type": "GROUP",
//   "last_read_id": 9150005005,
//   "last_message_id": 9150005005,
//   "moderated": false,
//   "users": [
//     2,
//     3,
//     102
//   ]
// }

// Represents an individual chat "channel" in the game.
// Field 	Type 	Description
// channel_id 	integer
// name 	string
// description 	string?
// icon 	string? 	display icon for the channel
// type 	ChannelType 	type of channel
// message_length_limit 	integer
// moderated 	boolean 	user can't send message when the value is true
// uuid 	string? 	value from requests that is relayed back to the sender.

// Optional attributes:
// Field 	Type 	Description
// current_user_attributes 	CurrentUserAttributes? 	only present on some responses
// last_read_id 	integer? 	Deprecated; use current_user_attributes.last_read_id.
// last_message_id 	integer? 	message_id of last known message (only returned in presence responses)
// recent_messages 	ChatMessage[]? 	Deprecated; up to 50 most recent messages
// users 	integer[]? 	array of user_id that are in the channel (not included for PUBLIC channels)

use serde::{Deserialize, Serialize};

use crate::v2::model::chat::structs::message::ChatMessage;
use crate::v2::model::comment::structs::meta::CurrentUserAttributes;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatChannel {
    pub channel_id: u32,
    pub current_user_attributes: Option<CurrentUserAttributes>,
    pub name: String,
    pub description: String,
    pub icon: String,
    #[serde(rename = "type")]
    pub channel_type: String,
    pub message_length_limit: u32,
    pub moderated: bool,
    pub uuid: String,
    pub last_read_id: Option<u64>,
    pub last_message_id: Option<u64>,
    pub recent_messages: Option<Vec<ChatMessage>>,
    pub users: Option<Vec<u32>>,
}
