// src/structs/user.rs

use crate::v2::model::user::enums::playstyle::Playstyle;
use crate::v2::model::user::enums::profile_order::ProfileOrder;
use crate::v2::model::user::structs::country::Country;
use crate::v2::model::user::structs::cover::Cover;
use crate::v2::model::user::structs::daily_challenge_user_stats::DailyChallengeUserStats;
use crate::v2::model::user::structs::group::Group;
use crate::v2::model::user::structs::kudosu::Kudosu;
use crate::v2::model::user::structs::monthly_playcounts::MonthlyPlaycounts;
use crate::v2::model::user::structs::rank_highest::RankHighest;
use crate::v2::model::user::structs::rank_history::RankHistory;
use crate::v2::model::user::structs::replays_watched_count::ReplaysWatchedCount;
use crate::v2::model::user::structs::statistics::Statistics;
use crate::v2::model::user::structs::statistics_rulesets::StatisticsRulesets;
use crate::v2::model::user::structs::user_achievements::UserAchievements;
use crate::v2::model::user::structs::team::Team;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: Option<String>,
    pub id: u32,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,

    pub last_visit: Option<String>,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,
    pub cover_url: Option<String>,
    pub discord: Option<String>,
    pub has_supported: Option<bool>,
    pub interests: Option<String>,

    pub join_date: Option<String>,
    pub location: Option<String>,
    pub max_blocks: Option<u32>,
    pub max_friends: Option<u32>,
    pub occupation: Option<String>,
    pub playmode: Option<String>,
    pub playstyle: Option<Vec<Playstyle>>,
    pub post_count: Option<u32>,
    pub profile_hue: Option<u32>,
    pub profile_order: Option<Vec<ProfileOrder>>,
    pub title: Option<String>,
    pub title_url: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub country: Option<Country>,
    pub cover: Option<Cover>,
    pub is_restricted: Option<bool>,
    pub kudosu: Option<Kudosu>,
    pub account_history: Option<Vec<String>>, // Assuming this is an empty array
    pub active_tournament_banner: Option<String>,
    pub active_tournament_banners: Option<Vec<String>>, // Assuming this is an empty array
    pub badges: Option<Vec<String>>,                    // Assuming this is an empty array
    pub beatmap_playcounts_count: Option<u32>,
    pub comments_count: Option<u32>,
    pub daily_challenge_user_stats: Option<DailyChallengeUserStats>,
    pub favourite_beatmapset_count: Option<u32>,
    pub follower_count: Option<u32>,
    pub graveyard_beatmapset_count: Option<u32>,
    pub groups: Option<Vec<Group>>, // Assuming this is an empty array
    pub guest_beatmapset_count: Option<u32>,
    pub loved_beatmapset_count: Option<u32>,
    pub mapping_follower_count: Option<u32>,
    pub monthly_playcounts: Option<Vec<MonthlyPlaycounts>>,
    pub nominated_beatmapset_count: Option<u32>,
    pub page: Option<Page>,
    pub pending_beatmapset_count: Option<u32>,
    pub previous_usernames: Option<Vec<String>>,
    pub rank_highest: Option<RankHighest>,
    pub ranked_beatmapset_count: Option<u32>,
    pub replays_watched_counts: Option<Vec<ReplaysWatchedCount>>, // Assuming this is an empty array
    pub scores_best_count: Option<u32>,
    pub scores_first_count: Option<u32>,
    pub scores_pinned_count: Option<u32>,
    pub scores_recent_count: Option<u32>,
    pub session_verified: Option<bool>,
    pub statistics: Option<Statistics>,
    pub statistics_rulesets: Option<StatisticsRulesets>,
    pub support_level: Option<u32>,
    pub team: Option<Team>,
    pub user_achievements: Option<Vec<UserAchievements>>,
    pub rank_history: Option<RankHistory>,
    #[serde(rename = "rankHistory")] // 好没素质
    pub rank_istoriya: Option<RankHistory>,
    pub ranked_and_approved_beatmapset_count: Option<u32>,
    pub unranked_beatmapset_count: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub html: String,
    pub raw: String,
}
