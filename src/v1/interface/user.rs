use crate::error::Result;
use crate::v1::model::user::{User,GetUserParams};
use crate::v1::model::best::{BestScore, GetUserBestParams};
use crate::v1::model::recent::{RecentPlay, GetUserRecentParams};
pub trait IUser {
    fn get_user(
        &self,
        params: GetUserParams,
    ) -> impl std::future::Future<Output = Result<User>> + Send;
    fn get_user_best(
        &self,
        params: GetUserBestParams,
    ) -> impl std::future::Future<Output = Result<Vec<BestScore>>> + Send;
    fn get_user_recent(
        &self,
        params: GetUserRecentParams,
    ) -> impl std::future::Future<Output = Result<Vec<RecentPlay>>> + Send;
}