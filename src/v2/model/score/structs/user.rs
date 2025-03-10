use serde::{Deserialize, Serialize};

use crate::v2::model::score::enums::country::Country;
use crate::v2::model::score::enums::cover::Cover;
use crate::v2::model::score::structs::team::Team;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
    pub id: u32,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: String,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,
    pub country: Country,
    pub cover: Cover,
    pub team: Option<Team>,
}
