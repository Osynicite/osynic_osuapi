// ChatMessage

// {
//   "channel_id": 5,
//   "content": "i am a lazerface",
//   "is_action": false,
//   "message_id": 9150005004,
//   "sender_id": 2,
//   "timestamp": "2018-07-06T06:33:34+00:00",
//   "type": "plain",
//   "uuid": "some-uuid-string",
//   "sender": {
//     "id": 2,
//     "username": "peppy",
//     "profile_colour": "#3366FF",
//     "avatar_url": "https://a.ppy.sh/2?1519081077.png",
//     "country_code": "AU",
//     "is_active": true,
//     "is_bot": false,
//     "is_online": true,
//     "is_supporter": true
//   }
// }

// Represents an individual Message within a ChatChannel.
// Field 	Type 	Description
// channel_id 	integer 	channel_id of where the message was sent
// content 	string 	message content
// is_action 	boolean 	was this an action? i.e. /me dances
// message_id 	integer 	unique identifier for message
// sender_id 	integer 	user_id of the sender
// timestamp 	Timestamp 	when the message was sent, ISO-8601
// type 	string 	type of message; 'action', 'markdown' or 'plain'
// uuid 	string? 	message identifier originally sent by client

// Optional attributes:
// Field 	Type 	Description
// sender 	User 	embedded User object to save additional api lookups

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub channel_id: u32,
    pub content: String,
    pub is_action: bool,
    pub message_id: u64,
    pub sender_id: u32,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub uuid: String,
    pub sender: Option<Sender>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sender {
    pub id: u32,
    pub username: String,
    pub profile_colour: Option<String>,
    pub avatar_url: String,
    pub country_code: String,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_online: bool,
    pub is_supporter: bool,
}
