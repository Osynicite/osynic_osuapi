use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentNomination {
    beatmapset_id: u32,
    rulesets: Vec<String>,
    reset: bool,
    user_id: u32,
}
