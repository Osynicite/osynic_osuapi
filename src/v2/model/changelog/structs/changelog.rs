// ChangelogListing

use serde::{Deserialize, Serialize};

use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::search::Search;
use crate::v2::model::changelog::structs::stream::Stream;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangelogListing {
    pub streams: Vec<Stream>,
    pub builds: Vec<ChanglogBuild>,
    pub search: Search,
}
