use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd,
)]
pub enum Scope {
    ChatRead,
    ChatWrite,
    ChatWriteMessage,
    Delegrate,
    ForumWrite,
    FriendRead,
    Identify,
    #[default]
    Public,
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Scope::ChatRead => "chat.read",
            Scope::ChatWrite => "chat.write",
            Scope::ChatWriteMessage => "chat.write.message",
            Scope::Delegrate => "delegate",
            Scope::ForumWrite => "forum.write",
            Scope::FriendRead => "friend.read",
            Scope::Identify => "identify",
            Scope::Public => "public",
        }
    }
    pub fn to_param(&self) -> String {
        match self {
            Scope::ChatRead => "chat.read".to_string(),
            Scope::ChatWrite => "chat.write".to_string(),
            Scope::ChatWriteMessage => "chat.write.message".to_string(),
            Scope::Delegrate => "delegate".to_string(),
            Scope::ForumWrite => "forum.write".to_string(),
            Scope::FriendRead => "friend.read".to_string(),
            Scope::Identify => "identify".to_string(),
            Scope::Public => "public".to_string(),
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Scope::ChatRead => write!(f, "chat.read"),
            Scope::ChatWrite => write!(f, "chat.write"),
            Scope::ChatWriteMessage => write!(f, "chat.write.message"),
            Scope::Delegrate => write!(f, "delegate"),
            Scope::ForumWrite => write!(f, "forum.write"),
            Scope::FriendRead => write!(f, "friend.read"),
            Scope::Identify => write!(f, "identify"),
            Scope::Public => write!(f, "public"),
        }
    }
}
