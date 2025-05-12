//   "search": {
//     "stream": null,
//     "from": null,
//     "to": null,
//     "max_id": null,
//     "limit": 21
//   }
// search.from 	string? 	from input.
// search.limit 	integer 	Always 21.
// search.max_id 	integer? 	max_id input.
// search.stream 	string? 	stream input.
// search.to 	string?         to input.

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    pub stream: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub max_id: Option<i64>,
    pub limit: i64,
}
