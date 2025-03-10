use crate::error::Result;
use crate::v2::model::wiki::WikiPage;

pub trait IWiki {
    fn get_wiki_page(
        &self,
        locale: String,
        path: String,
    ) -> impl std::future::Future<Output = Result<WikiPage>> + Send;
}
