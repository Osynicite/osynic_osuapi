use serde::{Deserialize, Serialize};

use super::user_silence::UserSilence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Silences {
    pub silences: Vec<UserSilence>,
}