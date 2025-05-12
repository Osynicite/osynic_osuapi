use serde::{Deserialize, Serialize};

use super::spotlight::Spotlight;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spotlights {
    pub spotlights: Vec<Spotlight>,
}
