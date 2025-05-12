// Body Parameters
// channel   object  optional  

// channel details; required if type is ANNOUNCE.
// name   string  optional  

// the channel name; required if type is ANNOUNCE.
// description   string  optional  

// the channel description; required if type is ANNOUNCE.
// message   string  optional  

// message to send with the announcement; required if type is ANNOUNCE.
// target_id   integer  optional  

// target user id; required if type is PM; ignored, otherwise.
// target_ids   integer[]  optional  

// target user ids; required if type is ANNOUNCE; ignored, otherwise.
// type   string   

// channel type (currently only supports PM and ANNOUNCE)

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateChannelParams {
    pub channel: Option<Channel>,
    pub message: Option<String>,
    pub target_id: Option<u32>,
    pub target_ids: Option<Vec<u32>>,
    #[serde(rename = "type")]
    pub channel_type: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub id: Option<u32>,
    pub name: Option<String>,
 }

 impl CreateChannelParams {
    pub fn new(
        channel: Option<Channel>,
        message: Option<String>,
        target_id: Option<u32>,
        target_ids: Option<Vec<u32>>,
        channel_type: String,
    ) -> Self {
        Self {
            channel,
            message,
            target_id,
            target_ids,
            channel_type,
        }
    }

    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn channel_id(mut self, channel_id: u32) -> Self {
        if let Some(ref mut channel) = self.channel {
            channel.id = Some(channel_id);
        } else {
            self.channel = Some(Channel {
                id: Some(channel_id),
                name: None,
            });
        }
        self
    }

    pub fn channel_name(mut self, channel_name: String) -> Self {
        if let Some(ref mut channel) = self.channel {
            channel.name = Some(channel_name);
        } else {
            self.channel = Some(Channel {
                id: None,
                name: Some(channel_name),
            });
        }
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn target_id(mut self, target_id: u32) -> Self {
        self.target_id = Some(target_id);
        self
    }

    pub fn target_ids(mut self, target_ids: Vec<u32>) -> Self {
        self.target_ids = Some(target_ids);
        self
    }

    pub fn channel_type(mut self, channel_type: String) -> Self {
        self.channel_type = channel_type;
        self
    }
}