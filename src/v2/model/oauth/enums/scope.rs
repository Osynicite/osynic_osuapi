#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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
