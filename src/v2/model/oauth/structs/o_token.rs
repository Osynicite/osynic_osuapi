use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OToken {
    pub access_token: String,
    pub expires_in: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub token_type: String,
}

impl Default for OToken {
    fn default() -> Self {
        OToken {
            access_token: "".to_string(),
            expires_in: 86400,
            refresh_token: None,
            token_type: "".to_string(),
        }
    }
}
