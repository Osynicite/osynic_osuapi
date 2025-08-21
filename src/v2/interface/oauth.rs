use crate::error::Result;
use crate::v2::model::oauth::enums::scope::Scope;
use crate::v2::model::oauth::structs::o_token::OToken;

pub trait IOauth {
    fn get_token_with_code(
        &self,
        client_id: u64,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> impl std::future::Future<Output = Result<OToken>>;
    fn get_token_without_code(
        &self,
        client_id: u64,
        client_secret: &str,
    ) -> impl std::future::Future<Output = Result<OToken>>;
    fn refresh_token(
        &self,
        client_id: u64,
        client_secret: &str,
        scope: Option<Vec<Scope>>,
    ) -> impl std::future::Future<Output = Result<OToken>>;
    fn revoke_current_token(&self) -> impl std::future::Future<Output = Result<()>>;
}
