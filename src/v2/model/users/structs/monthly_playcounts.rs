// src/structs/monthly_playcounts.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyPlaycounts {
    
    pub start_date: String,
    pub count: u64,
}