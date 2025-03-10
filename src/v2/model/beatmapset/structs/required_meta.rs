use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredMeta {
    pub main_ruleset: u32,
    pub non_main_ruleset: u32,
}
