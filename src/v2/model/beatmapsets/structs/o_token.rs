use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OToken{
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OCToken{
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}