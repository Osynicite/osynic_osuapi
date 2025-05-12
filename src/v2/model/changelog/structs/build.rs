// {
//       "id": 5823,
//       "version": "2021.619.1",
//       "display_version": "2021.619.1",
//       "users": 0,
//       "created_at": "2021-06-19T08:30:45+00:00",
//       "update_stream": {
//         "id": 7,
//         "name": "lazer",
//         "display_name": "Lazer",
//         "is_featured": false
//       },
//       "changelog_entries": [
//         {
//           "id": 12925,
//           "repository": "ppy/osu",
//           "github_pull_request_id": 13572,
//           "github_url": "https://github.com/ppy/osu/pull/13572",
//           "url": null,
//           "type": "fix",
//           "category": "Reliability",
//           "title": "Fix game crashes due to attempting localisation load for unsupported locales",
//           "message_html": null,
//           "major": true,
//           "created_at": "2021-06-19T08:09:39+00:00",
//           "github_user": {
//             "display_name": "bdach",
//             "github_url": "https://github.com/bdach",
//             "github_username": "bdach",
//             "id": 218,
//             "osu_username": null,
//             "user_id": null,
//             "user_url": null
//           }
//         }
//       ]
//     },

use serde::{Deserialize, Serialize};

use crate::v2::model::changelog::structs::stream::UpdateStream;
use crate::v2::model::changelog::structs::versions::Versions;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChanglogBuild {
    pub id: i64,
    pub display_version: String,
    pub users: i64,
    pub created_at: String,
    pub update_stream: UpdateStream,
    pub version: Option<String>,
    pub youtube_id: Option<String>,
    pub changelog_entries: Option<Vec<ChangelogEntry>>,
    pub versions: Option<Versions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub id: Option<i64>,
    pub repository: Option<String>,
    pub github_pull_request_id: Option<i64>,
    pub github_url: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub category: String,
    pub title: Option<String>,
    pub message: Option<String>,
    pub message_html: Option<String>,
    pub major: bool,
    pub created_at: Option<String>,
    pub github_user: Option<GithubUser>,
    pub versions: Option<Versions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubUser {
    pub display_name: String,
    pub github_url: Option<String>,
    pub github_username: Option<String>,
    pub id: Option<i64>,
    pub osu_username: Option<String>,
    pub user_id: Option<i64>,
    pub user_url: Option<String>,
}