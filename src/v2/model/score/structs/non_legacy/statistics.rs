use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub great: u32,
    pub large_tick_hit: u32,
    pub small_tick_hit: u32,
}
