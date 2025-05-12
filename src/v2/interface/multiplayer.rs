use crate::error::Result;
use crate::v2::model::room::structs::room::Room;
use crate::v2::model::score::structs::multiplayer::multiplayer_scores::MultiplayerScores;
use crate::v2::model::score::structs::score::Score;

pub trait IMultiplayer {
    fn get_user_high_score(
        &self,
        room: String,
        playlist: u64,
        user: u64,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn get_scores(
        &self,
        room: String,
        playlist: u64,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<MultiplayerScores>> + Send;
    fn get_score(
        &self,
        room: String,
        playlist: u64,
        score: u64,
    ) -> impl std::future::Future<Output = Result<Score>> + Send;
    fn get_multiplayer_rooms(
        &self,
        limit: Option<u32>,
        mode: Option<String>,
        season_id: Option<u32>,
        sort: Option<String>,
        type_group: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<Room>>> + Send;
}
