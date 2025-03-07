// src/structs/monthly_playcounts.rs

use serde::{Serialize, Deserialize};

#[derive(Default,Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct MonthlyPlaycounts {
    
    pub start_date: String,
    pub count: u32,
}