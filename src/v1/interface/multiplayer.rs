use crate::error::Result;
use crate::v1::model::multiplayer::{GetMatchParams, MultiplayerResponse};

pub trait IMultiplayer {
    fn get_match(
        &self,
        params: GetMatchParams,
    ) -> impl std::future::Future<Output = Result<MultiplayerResponse>>;
}
