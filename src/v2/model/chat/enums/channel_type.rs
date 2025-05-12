// ChannelType
// Type 	Permission Check for Joining/Messaging
// PUBLIC 	
// PRIVATE 	is player in the allowed groups? (channel.allowed_groups)
// MULTIPLAYER 	is player currently in the mp game?
// SPECTATOR 	
// TEMPORARY 	deprecated
// PM 	see below (user_channels)
// GROUP 	is player in channel? (user_channels)
// ANNOUNCE 	is user in the announce group?

use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd,
)]
pub enum ChannelType {
    #[default]
    /// Public channel
    Public,
    /// Private channel
    Private,
    /// Multiplayer channel
    Multiplayer,
    /// Spectator channel
    Spectator,
    /// Temporary channel
    Temporary,
    /// Private message channel
    PM,
    /// Group channel
    Group,
    /// Announcement channel
    Announce,
}

impl ChannelType {
    /// Returns the string representation of the channel type.
    pub fn as_str(&self) -> &'static str {
        match self {
            ChannelType::Public => "PUBLIC",
            ChannelType::Private => "PRIVATE",
            ChannelType::Multiplayer => "MULTIPLAYER",
            ChannelType::Spectator => "SPECTATOR",
            ChannelType::Temporary => "TEMPORARY",
            ChannelType::PM => "PM",
            ChannelType::Group => "GROUP",
            ChannelType::Announce => "ANNOUNCE",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}