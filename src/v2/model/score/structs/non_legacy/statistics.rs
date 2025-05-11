use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub miss: Option<u32>,
    pub great: Option<u32>,
    pub large_tick_hit: Option<u32>,
    pub small_tick_hit: Option<u32>,
    pub small_tick_miss: Option<u32>,
}
