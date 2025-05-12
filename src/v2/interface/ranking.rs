// Placeholder
use crate::error::Result;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::ranking::enums::ranking_type::RankingType;
use crate::v2::model::ranking::structs::rankings::{KudosuRankings, Rankings};
use crate::v2::model::ranking::structs::spotlights::Spotlights;
pub trait IRanking {
    fn get_kudosu_ranking(
        &self,
        page: Option<u32>,
    ) -> impl std::future::Future<Output = Result<KudosuRankings>> + Send;

    fn get_ranking(
        &self,
        mode: Mode,
        ranking_type: RankingType,
        country: Option<String>,
        cursor_string: Option<String>,
        filter: Option<String>,
        spotlight: Option<String>,
        variant: Option<String>,
    ) -> impl std::future::Future<Output = Result<Rankings>> + Send;

    fn get_spotlights(&self) -> impl std::future::Future<Output = Result<Spotlights>> + Send;
}
