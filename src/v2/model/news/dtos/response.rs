use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNewsListingResponse {
    pub news_posts: Vec<NewsPost>,
    pub news_sidebar: NewsSidebar,
    pub search: Search,
    pub cursor_string: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsPost {
    pub id: u64,
    pub author: String,
    pub edit_url: String,
    pub first_image: String,
    pub published_at: String,
    pub updated_at: String,
    pub slug: String,
    pub title: String,
    pub preview: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsSidebar {
    pub current_year: u32,
    pub news_posts: Vec<NewsPost>,
    pub years: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    pub limit: u32,
    pub sort: String,
}

// {
//   "news_posts": [
//     {
//       "id": 964,
//       "author": "RockRoller",
//       "edit_url": "https://github.com/ppy/osu-wiki/tree/master/news/2021-05-27-skinning-contest-results.md",
//       "first_image": "https://i.ppy.sh/d431ff921955d5c8792dc9bae40ac082d4e53131/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032312d30352d32372d736b696e6e696e672d636f6e746573742d726573756c74732f736b696e6e696e675f636f6e746573745f62616e6e65722e6a7067",
//       "published_at": "2021-05-27T12:00:00+00:00",
//       "updated_at": "2021-05-28T17:11:35+00:00",
//       "slug": "2021-05-27-skinning-contest-results",
//       "title": "Skinning Contest: Results Out",
//       "preview": "The ship full of skins is now back with your votes. Check out the results for our first-ever official skinning contest right here!"
//     },
//     // ...
//   ],
//   "news_sidebar": {
//     "current_year": 2021,
//     "news_posts": [
//       {
//         "id": 964,
//         "author": "RockRoller",
//         "edit_url": "https://github.com/ppy/osu-wiki/tree/master/news/2021-05-27-skinning-contest-results.md",
//         "first_image": "https://i.ppy.sh/d431ff921955d5c8792dc9bae40ac082d4e53131/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032312d30352d32372d736b696e6e696e672d636f6e746573742d726573756c74732f736b696e6e696e675f636f6e746573745f62616e6e65722e6a7067",
//         "published_at": "2021-05-27T12:00:00+00:00",
//         "updated_at": "2021-05-28T17:11:35+00:00",
//         "slug": "2021-05-27-skinning-contest-results",
//         "title": "Skinning Contest: Results Out"
//       },
//       // ...
//     ],
//     "years": [2021, 2020, 2019, 2018, 2017, 2016, 2015, 2014, 2013]
//   },
//   "search": {
//     "limit": 12,
//     "sort": "published_desc"
//   },
//   "cursor_string": "WyJodHRwczpcL1wvd3d3LnlvdXR1YmUuY29tXC93YXRjaD92PWRRdzR3OVdnWGNRIl0"
// }
