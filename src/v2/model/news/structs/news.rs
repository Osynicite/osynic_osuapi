use serde::{Deserialize, Serialize};

use crate::v2::model::news::structs::navigation::Navigation;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct News {
    pub id: u64,
    pub author: String,
    pub edit_url: String,
    pub first_image: String,
    pub published_at: String,
    pub updated_at: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub navigation: Navigation,
}

// {
//     "id": 943,
//     "author": "pishifat",
//     "edit_url": "https://github.com/ppy/osu-wiki/tree/master/news/2021-04-27-results-a-labour-of-love.md",
//     "first_image": "https://i.ppy.sh/65c9c2eb2f8d9bc6008b95aba7d0ef45e1414c1e/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f7368617265642f6e6577732f323032302d31312d33302d612d6c61626f75722d6f662d6c6f76652f616c6f6c5f636f7665722e6a7067",
//     "published_at": "2021-04-27T20:00:00+00:00",
//     "updated_at": "2021-04-27T20:25:57+00:00",
//     "slug": "2021-04-27-results-a-labour-of-love",
//     "title": "Results - A Labour of Love",
//     "content": "<div class='osu-md osu-md--news'>...</div>",
//     "navigation": {