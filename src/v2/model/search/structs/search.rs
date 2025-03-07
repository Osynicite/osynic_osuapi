
use crate::v2::model::search::enums::sort::Sort;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
pub struct Search{
    pub sort: String,
}

impl From<Sort> for Search {
    fn from(sort: Sort) -> Search {
        Search{
            sort: sort.to_beatmapset_search()
        }
    }
}

impl Default for Search {
    fn default() -> Search {
        Search{
            sort: "relevance_dsec".to_string()
        }
    }
}