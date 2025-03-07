use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct CurrentUserAttributes {
    pub pin: Option<String>,
}