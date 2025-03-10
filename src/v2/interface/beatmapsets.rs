use crate::error::Result;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::search::dtos::params::BeatmapsetsSearchParams;
use crate::v2::model::search::dtos::response::BeatmapsetsSearchResponse;

pub trait IBeatmapsets {
    fn search(
        &self,
        params: BeatmapsetsSearchParams,
    ) -> impl std::future::Future<Output = Result<BeatmapsetsSearchResponse>> + Send;
    // fn lookup(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    fn get_beatmapset(
        &self,
        beatmapset_id: u32,
    ) -> impl std::future::Future<Output = Result<Beatmapset>> + Send;
    fn download(&self, beatmapset_id: u32) -> impl std::future::Future<Output = Result<()>> + Send;
    // {"authentication":"basic"}
}
