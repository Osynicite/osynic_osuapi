use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Availability {
    pub download_disabled: bool,
    pub more_information: Option<String>,
}
