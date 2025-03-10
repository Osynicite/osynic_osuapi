use crate::error::Result;
use crate::v2::model::search::dtos::response::SearchResponse;
use crate::v2::model::search::enums::search_mode::SearchMode;

pub trait ISearch {
    fn search(
        &self,
        mode: Option<SearchMode>,
        query: Option<String>,
        page: Option<u32>,
    ) -> impl std::future::Future<Output = Result<SearchResponse>> + Send;
}
