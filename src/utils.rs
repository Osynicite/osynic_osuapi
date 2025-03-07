
use crate::v2::model::user::structs::user::User;
use serde::{Serialize, Deserialize};

// e.g. CN -> 1f1e8-1f1f3, for /assets/images/flags/1f1e8-1f1f3.svg
pub fn country_code_to_unicode_flag(code: &str) -> Option<String> {
    if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
        let base = 0x1F1E6 - 'A' as u32;
        let flag = code.to_uppercase()
            .chars()
            .map(|c| format!("{:x}", base + c as u32))
            .collect::<Vec<_>>()
            .join("-");
        Some(flag)
    } else {
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct OsuAccountFacadeA {
    pub id: u32,
    pub remember_token: bool,
    pub username: String,
    pub avatar_url: String,
    pub cover_url: String,
    pub country_svg_url: String,
    pub accuracy: f64,
    pub level: u32,
    pub progress: u32,
    pub pp: f64,
    pub world_rank: u32,
    pub country_code: String,
    pub country_rank: u32,
    pub is_supporter: bool,
}

pub fn osu_account_facade(user: User) -> OsuAccountFacadeA {
    let country_flag = country_code_to_unicode_flag(&user.country.code).unwrap_or_default();
    let country_svg_url = format!("https://osu.ppy.sh/assets/images/flags/{}.svg", country_flag);
    OsuAccountFacadeA {
        id: user.id,
        remember_token: false,
        username: user.username,
        avatar_url: user.avatar_url,
        cover_url: user.cover_url,
        country_svg_url,
        accuracy: user.statistics.hit_accuracy,
        level: user.statistics.level.current,
        progress: user.statistics.level.progress,
        pp: user.statistics.pp,
        world_rank: user.statistics.global_rank,
        country_code: user.country.code,
        country_rank: user.statistics.country_rank.unwrap_or_default(),
        is_supporter: user.is_supporter,
    }
}