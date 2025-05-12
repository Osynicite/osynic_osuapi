// "versions": {
//         "previous": {
//             "id": 5774,
//             "version": "20210519.3",
//             "display_version": "20210519.3",
//             "users": 10,
//             "created_at": "2021-05-19T11:51:48+00:00",
//             "update_stream": {
//                 "id": 5,
//                 "name": "stable40",
//                 "display_name": "Stable",
//                 "is_featured": true
//             }
//         }
//     }

use serde::{Deserialize, Serialize};
use crate::v2::model::changelog::structs::stream::UpdateStream;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Versions {
    pub next: Option<VersionsBuild>,
    pub previous: Option<VersionsBuild>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionsBuild {
    pub id: i64,
    pub version: String,
    pub display_version: String,
    pub users: i64,
    pub created_at: String,
    pub update_stream: UpdateStream,
}


