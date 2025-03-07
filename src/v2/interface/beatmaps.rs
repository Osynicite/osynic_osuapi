use crate::error::Result;
use crate::v2::model::beatmap::structs::beatmap::Beatmap;

pub trait IBeatmaps {
    // fn lookup(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn get_user_score(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn get_user_scores(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn get_scores(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn get_solo_scores(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    // fn get_beatmaps(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    fn get_beatmap(&self,beatmap_id:u32) -> impl std::future::Future<Output = Result<Beatmap>> + Send;
    // fn get_beatmap_attributes(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}