use crate::error::Result;
use crate::v2::model::beatmap::structs::beatmap_playcount::BeatmapPlaycount;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::event::structs::event::Event;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::score::enums::score_type::ScoreType;
use crate::v2::model::score::structs::score::Score;
use crate::v2::model::user::structs::kudosu_history::KudosuHisotry;
use crate::v2::model::user::structs::user::User;
use crate::v2::model::user::structs::users::Users;

pub trait IUsers {
    fn get_own_data(
        &self,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> impl std::future::Future<Output = Result<User>> + Send;
    fn get_user_kudosu(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<KudosuHisotry>>> + Send;
    fn get_user_scores(
        &self,
        id: u32,
        score_type: ScoreType,
        legacy_only: Option<u32>,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<Score>>> + Send;
    fn get_user_beatmaps(
        &self,
        id: u32,
        beatmap_type: String,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<Beatmapset>>> + Send;
    fn get_user_beatmaps_most_played(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<BeatmapPlaycount>>> + Send;
    fn get_user_recent_activity(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<Event>>> + Send;
    fn get_user(
        &self,
        id: u32,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> impl std::future::Future<Output = Result<User>> + Send;
    fn get_user_by_username(
        &self,
        username: &str,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> impl std::future::Future<Output = Result<User>> + Send;
    fn get_users(
        &self,
        ids: Vec<u32>,
        include_variant_statistics: Option<bool>,
    ) -> impl std::future::Future<Output = Result<Users>> + Send;
}
