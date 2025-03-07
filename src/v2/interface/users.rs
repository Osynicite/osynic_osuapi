use crate::error::Result;
use crate::v2::model::user::structs::user::User;
use crate::v2::model::mode::enums::mode::Mode;

pub trait IUsers {
    fn get_own_data(&self,mode: Option<Mode>) -> impl std::future::Future<Output = Result<User>> + Send;
    fn get_user(&self,id: &str,mode: Option<Mode>) -> impl std::future::Future<Output = Result<User>> + Send;
}