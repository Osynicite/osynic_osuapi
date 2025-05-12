use crate::error::Result;
use crate::v1::model::multiplayer::MultiplayerScore;
use crate::v2::model::score::structs::score::Score;

pub trait IComments {
    fn get_comments(
        &self,
        after: Option<String>,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        cursor: Option<String>,
        parent_id: Option<String>,
        sort: Option<String>,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn post_comment(
        &self,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        parent_id: Option<String>,
        message: Option<String>,
    ) -> impl std::future::Future<Output = Result<MultiplayerScore>> + Send;
    fn get_comment(
        &self,
        comment: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn edit_comment(
        &self,
        comment: String,
        message: Option<String>,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn delete_comment(
        &self,
        comment: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn add_comment_vote(
        &self,
        comment: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn remove_comment_vote(
        &self,
        comment: String,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;

}
