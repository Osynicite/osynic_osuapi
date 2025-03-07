use serde::{Serialize, Deserialize};

use super::required_meta::RequiredMeta;

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct NominationsSummary {
    pub current: u32,
    pub eligible_main_rulesets: Vec<String>,
    pub required_meta: RequiredMeta,
}