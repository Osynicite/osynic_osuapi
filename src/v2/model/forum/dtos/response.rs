use serde::{Deserialize, Serialize};

use crate::v2::model::forum::structs::forum::Forum;
use crate::v2::model::forum::structs::post::ForumPost;
use crate::v2::model::forum::structs::topic::ForumTopic;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTopicResponse {
    pub topic: ForumTopic,
    pub post: ForumPost,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTopicAndPostsResponse {
    pub cursor_string: String,
    pub posts: Vec<ForumPost>,
    pub search: Search,
    pub topic: ForumTopic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    pub limit: u32,
    pub sort: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetForumAndTopicsResponse {
    pub forum: Forum,
    pub topics: Vec<ForumTopic>,
    pub pinned_topics: Vec<ForumTopic>,
}