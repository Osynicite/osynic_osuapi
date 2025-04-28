use crate::error::Result;
use crate::v1::model::scores::{Score,GetScoresParams};

pub trait IScores {
    fn get_scores(
        &self,
        params: GetScoresParams,
    ) -> impl std::future::Future<Output = Result<Vec<Score>>> + Send;
}