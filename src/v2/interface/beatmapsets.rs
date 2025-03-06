use crate::error::Result;
use crate::v2::model::beatmapsets::structs::search::BeatmapsetsSearchParams;

pub trait IBeatmapsets {
    fn search(&self, params: BeatmapsetsSearchParams) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn lookup(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn download(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn get_beatmapset(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}