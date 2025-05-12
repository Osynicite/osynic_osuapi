// Placeholder
use crate::error::Result;
use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::changelog::ChangelogListing;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::ranking::enums::ranking_type::RankingType;

pub trait IRanking {
    fn get_kudosu_ranking(
        &self,
        page: Option<u32>,
    ) -> impl std::future::Future<Output = Result<ChangelogListing>> + Send;

    fn get_ranking(
        &self,
        mode: Mode,
        ranking_type: RankingType,
        country: Option<String>,
        cursor_string: Option<String>,
        filter: Option<String>,
        spotlight: Option<String>,
        variant: Option<String>,
    ) -> impl std::future::Future<Output = Result<ChanglogBuild>> + Send;
    
    fn lookup_spotlights(
        &self,
        changelog: String,
        key: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> impl std::future::Future<Output = Result<ChanglogBuild>> + Send;
}
