// Placeholder
use crate::error::Result;
use crate::v2::model::news::dtos::response::GetNewsListingResponse;
use crate::v2::model::news::structs::news::News;

pub trait INews {
    fn get_news_listing(
        &self,
        limit: Option<u32>,
        year: Option<u32>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<GetNewsListingResponse>> + Send;
    fn get_news_post(
        &self,
        news: String,
        key: Option<String>,
    ) -> impl std::future::Future<Output = Result<News>> + Send;
}
