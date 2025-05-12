// {
//   "spotlights": [
//     {
//       "end_date": "2019-03-22T00:00:00+00:00",
//       "id": 1,
//       "mode_specific": false,
//       "name": "Best spinning circles 2019",
//       "start_date": "2019-02-22T00:00:00+00:00",
//       "type": "yearly",
//     },
//     {
//       "end_date": "2019-03-22T00:00:00+00:00",
//       "id": 2,
//       "mode_specific": true,
//       "name": "Ultimate fruit collector February 2019",
//       "start_date": "2019-02-22T00:00:00+00:00",
//       "type": "monthly",
//     }
//   ],
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spotlight {
    pub end_date: String,
    pub id: u32,
    pub mode_specific: bool,
    pub name: String,
    pub start_date: String,
    #[serde(rename = "type")]
    pub spotlight_type: String,
}
