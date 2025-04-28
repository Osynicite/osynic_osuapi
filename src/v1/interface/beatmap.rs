use crate::error::Result;
use crate::v1::model::beatmap::{Beatmap, GetBeatmapsParams};

pub trait IBeatmap {
    fn get_beatmaps(
        &self,
        params: GetBeatmapsParams,
    ) -> impl std::future::Future<Output = Result<Vec<Beatmap>>> + Send;
}
