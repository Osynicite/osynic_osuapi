// "navigation": {
//         "newer": {
//             "id": 944,
//             "author": "pishifat",
//             "edit_url": "https://github.com/ppy/osu-wiki/tree/master/news/2021-04-28-new-featured-artist-emilles-moonlight-serenade.md",
//             "first_image": "https://i.ppy.sh/7e22cc5f4755c21574d999d8ce3a2f40a3268e84/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3136302f6865616465722e6a7067",
//             "published_at": "2021-04-28T08:00:00+00:00",
//             "updated_at": "2021-04-28T09:51:28+00:00",
//             "slug": "2021-04-28-new-featured-artist-emilles-moonlight-serenade",
//             "title": "New Featured Artist: Emille's Moonlight Serenade"
//         },
//         "older": {
//             "id": 942,
//             "author": "pishifat",
//             "edit_url": "https://github.com/ppy/osu-wiki/tree/master/news/2021-04-24-new-featured-artist-grynpyret.md",
//             "first_image": "https://i.ppy.sh/acdce813b71371b95e8240f9249c916285fdc5a0/68747470733a2f2f6173736574732e7070792e73682f617274697374732f3135392f6865616465722e6a7067",
//             "published_at": "2021-04-24T08:00:00+00:00",
//             "updated_at": "2021-04-24T10:23:59+00:00",
//             "slug": "2021-04-24-new-featured-artist-grynpyret",
//             "title": "New Featured Artist: Grynpyret"
//         }
//     }

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Navigation {
    pub newer: Option<Newer>,
    pub older: Option<Older>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Newer {
    pub id: u64,
    pub author: String,
    pub edit_url: String,
    pub first_image: String,
    pub published_at: String,
    pub updated_at: String,
    pub slug: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Older {
    pub id: u64,
    pub author: String,
    pub edit_url: String,
    pub first_image: String,
    pub published_at: String,
    pub updated_at: String,
    pub slug: String,
    pub title: String,
}
