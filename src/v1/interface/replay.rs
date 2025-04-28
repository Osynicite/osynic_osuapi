use crate::error::Result;
use crate::v1::model::replay::{GetReplayParams, Replay};

pub trait IReplay {
    fn get_replay(
        &self,
        params: GetReplayParams,
    ) -> impl std::future::Future<Output = Result<Replay>> + Send;
}
