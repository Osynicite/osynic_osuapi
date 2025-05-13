use crate::error::Result;
use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::beatmap::structs::beatmaps::Beatmaps;
use crate::v2::model::beatmap::structs::difficulty_attributes::Attributes;
use crate::v2::model::beatmap::structs::pack::BeatmapPack;
use crate::v2::model::beatmap::structs::packs::BeatmapPacks;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::score::structs::beatmap_scores::BeatmapScores;
use crate::v2::model::score::structs::beatmap_user_score::BeatmapUserScore;
use crate::v2::model::score::structs::non_legacy_scores::NonLegacyScores;
use crate::v2::model::score::structs::scores::Scores;

pub trait IBeatmaps {
    fn get_beatmap_packs(
        &self,
        pack_type: Option<String>,
        cursor_string: Option<String>,
    ) -> impl std::future::Future<Output = Result<BeatmapPacks>> + Send;
    fn get_beatmap_pack(
        &self,
        pack: String,
        legacy_only: Option<u32>,
    ) -> impl std::future::Future<Output = Result<BeatmapPack>> + Send;
    fn lookup(
        &self,
        checksum: Option<String>,
        filename: Option<String>,
        id: Option<String>,
    ) -> impl std::future::Future<Output = Result<Beatmap>> + Send;
    fn get_user_score(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
    ) -> impl std::future::Future<Output = Result<BeatmapUserScore>> + Send;
    fn get_user_scores(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        ruleset: Option<Mode>,
    ) -> impl std::future::Future<Output = Result<Scores>> + Send;
    fn get_scores(
        &self,
        beatmap_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> impl std::future::Future<Output = Result<BeatmapScores>> + Send;
    fn get_solo_scores(
        &self,
        beatmap_id: u32,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> impl std::future::Future<Output = Result<NonLegacyScores>> + Send;
    fn get_beatmaps(
        &self,
        beatmap_ids: Vec<u32>,
    ) -> impl std::future::Future<Output = Result<Beatmaps>> + Send;
    fn get_beatmap(
        &self,
        beatmap_id: u32,
    ) -> impl std::future::Future<Output = Result<Beatmap>> + Send;
    fn get_beatmap_attributes(
        &self,
        beatmap_id: u32,
        mods: Option<Vec<String>>,
        ruleset: Option<Mode>,
        ruleset_id: Option<i32>,
    ) -> impl std::future::Future<Output = Result<Attributes>> + Send;
}
