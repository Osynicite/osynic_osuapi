// {
//       "id": 5,
//       "name": "stable40",
//       "display_name": "Stable",
//       "is_featured": true,
//       "latest_build": {
//         "id": 5778,
//         "version": "20210520.2",
//         "display_version": "20210520.2",
//         "users": 23683,
//         "created_at": "2021-05-20T14:28:04+00:00",
//         "update_stream": {
//           "id": 5,
//           "name": "stable40",
//           "display_name": "Stable",
//           "is_featured": true
//         }
//       },
//       "user_count": 23965
//     },

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stream {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub is_featured: bool,
    pub latest_build: Option<LatestBuild>,
    pub user_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestBuild {
    pub id: i64,
    pub version: String,
    pub display_version: String,
    pub users: i64,
    pub created_at: String,
    pub update_stream: UpdateStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStream {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub is_featured: bool,
    pub user_count: Option<i64>,
}
