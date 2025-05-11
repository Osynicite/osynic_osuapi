// Placeholder
use crate::error::Result;
use crate::v2::model::matches::dtos::response::{GetMatchResponse, GetMatchesListingResponse};

pub trait IMatches {
    fn get_matches_listing(
        &self,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<GetMatchesListingResponse>> + Send;
    fn get_match(
        &self,
        match_id: u64,
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<GetMatchResponse>> + Send;
}
