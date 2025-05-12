use serde::{Deserialize, Serialize};

use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::ranking::structs::spotlight::Spotlight;
use crate::v2::model::user::structs::statistics::Statistics;
use crate::v2::model::user::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KudosuRankings {
    pub ranking: Vec<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rankings {
    pub beatmapsets: Option<Vec<Beatmapset>>,
    pub cursor: Cursor,
    pub ranking: Vec<Statistics>,
    pub spotlight: Option<Spotlight>,
    pub total: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub page: u64,
}
