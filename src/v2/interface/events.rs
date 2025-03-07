use crate::error::Result;
use crate::v2::model::event::dtos::response::GetEventsResponse;

pub trait IEvents {
    fn get_events(&self, sort: Option<String>,cursor_string: Option<String>) -> impl std::future::Future<Output = Result<GetEventsResponse>> + Send;
}