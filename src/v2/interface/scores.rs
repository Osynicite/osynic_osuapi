use crate::error::Result;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::score::dtos::response::GetScoresResponse;

pub trait IScores {
    fn get_scores(&self, ruleset: Option<Mode>,cursor_string: Option<String>) -> impl std::future::Future<Output = Result<GetScoresResponse>> + Send;
}