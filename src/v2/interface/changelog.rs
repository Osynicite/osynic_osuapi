// Placeholder
use crate::error::Result;
use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::changelog::ChangelogListing;

pub trait IChangelog {
    fn get_changelog_build(
        &self,
        stream: String,
        build: String,
    ) -> impl std::future::Future<Output = Result<ChanglogBuild>> + Send;
    
    fn get_changelog_listing(
        &self,
        from: Option<String>,
        max_id: Option<u32>,
        stream: Option<String>,
        to: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> impl std::future::Future<Output = Result<ChangelogListing>> + Send;
    
    fn lookup_changelog_build(
        &self,
        changelog: String,
        key: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> impl std::future::Future<Output = Result<ChanglogBuild>> + Send;
}
