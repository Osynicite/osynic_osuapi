use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::beatmaps::IBeatmaps;
use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::beatmap::structs::beatmaps::Beatmaps;
use crate::v2::model::beatmap::structs::difficulty_attributes::Attributes;
use crate::v2::model::beatmap::structs::pack::BeatmapPack;
use crate::v2::model::beatmap::structs::packs::BeatmapPacks;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::structs::beatmap_scores::BeatmapScores;
use crate::v2::model::score::structs::beatmap_user_score::BeatmapUserScore;
use crate::v2::model::score::structs::non_legacy_scores::NonLegacyScores;
use crate::v2::model::score::structs::scores::Scores;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooBeatmaps {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IBeatmaps for GlooBeatmaps {
    async fn get_beatmap_packs(
        &self,
        pack_type: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<BeatmapPacks> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_beatmap_packs"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(pack_type) = pack_type {
            query_params.push(("type", pack_type));
        }
        if let Some(cursor) = cursor_string {
            query_params.push(("cursor", cursor));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/packs")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/packs?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap_packs: BeatmapPacks = response.json().await?;
        Ok(beatmap_packs)
    }

    async fn get_beatmap_pack(
        &self,
        pack: String,
        legacy_only: Option<u32>,
    ) -> Result<BeatmapPack> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_beatmap_pack"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(legacy) = legacy_only {
            query_params.push(("legacy_only", legacy.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/packs/{pack}")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/packs/{pack}?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap_pack: BeatmapPack = response.json().await?;
        Ok(beatmap_pack)
    }

    async fn lookup(
        &self,
        checksum: Option<String>,
        filename: Option<String>,
        id: Option<String>,
    ) -> Result<Beatmap> {
        console::log_1(&JsValue::from_str("GlooBeatmaps lookup"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(checksum) = checksum {
            query_params.push(("checksum", checksum));
        }
        if let Some(filename) = filename {
            query_params.push(("filename", filename));
        }
        if let Some(id) = id {
            query_params.push(("id", id));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/lookup")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/lookup?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap: Beatmap = response.json().await?;
        Ok(beatmap)
    }

    async fn get_user_score(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
    ) -> Result<BeatmapUserScore> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_user_score"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(legacy) = legacy_only {
            query_params.push(("legacy_only", legacy.to_string()));
        }
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(mods) = mods {
            query_params.push(("mods", mods));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/scores/users/{user_id}"
            )
        } else {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/scores/users/{user_id}?{query_string}"
            )
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap_user_score: BeatmapUserScore = response.json().await?;
        Ok(beatmap_user_score)
    }

    async fn get_user_scores(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        ruleset: Option<Mode>,
    ) -> Result<Scores> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_user_scores"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(legacy) = legacy_only {
            query_params.push(("legacy_only", legacy.to_string()));
        }
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(ruleset) = ruleset {
            query_params.push(("ruleset", ruleset.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/scores/users/{user_id}/all"
            )
        } else {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/scores/users/{user_id}/all?{query_string}"
            )
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let scores: Scores = response.json().await?;
        Ok(scores)
    }

    async fn get_scores(
        &self,
        beatmap_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> Result<BeatmapScores> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_scores"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(legacy) = legacy_only {
            query_params.push(("legacy_only", legacy.to_string()));
        }
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(mods) = mods {
            query_params.push(("mods", mods));
        }
        if let Some(ranking_type) = ranking_type {
            query_params.push(("type", ranking_type));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/scores")
        } else {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/scores?{query_string}"
            )
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap_scores: BeatmapScores = response.json().await?;
        Ok(beatmap_scores)
    }

    async fn get_solo_scores(
        &self,
        beatmap_id: u32,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> Result<NonLegacyScores> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_solo_scores"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(mods) = mods {
            query_params.push(("mods", mods));
        }
        if let Some(ranking_type) = ranking_type {
            query_params.push(("type", ranking_type));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/solo-scores")
        } else {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/solo-scores?{query_string}"
            )
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let non_legacy_scores: NonLegacyScores = response.json().await?;
        Ok(non_legacy_scores)
    }

    async fn get_beatmaps(&self, beatmap_ids: Vec<u32>) -> Result<Beatmaps> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_beatmaps"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let ids_string = beatmap_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let query_params = vec![("ids", ids_string)];
        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps?{query_string}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmaps: Beatmaps = response.json().await?;
        Ok(beatmaps)
    }

    async fn get_beatmap(&self, beatmap_id: u32) -> Result<Beatmap> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_beatmap"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let beatmap: Beatmap = response.json().await?;
        Ok(beatmap)
    }

    async fn get_beatmap_attributes(
        &self,
        beatmap_id: u32,
        mods: Option<Vec<String>>,
        ruleset: Option<Mode>,
        ruleset_id: Option<i32>,
    ) -> Result<Attributes> {
        console::log_1(&JsValue::from_str("GlooBeatmaps get_beatmap_attributes"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(mods) = mods {
            for mod_name in mods {
                query_params.push(("mods[]", mod_name));
            }
        }
        if let Some(ruleset) = ruleset {
            query_params.push(("ruleset", ruleset.to_string()));
        }
        if let Some(ruleset_id) = ruleset_id {
            query_params.push(("ruleset_id", ruleset_id.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/attributes")
        } else {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/beatmaps/{beatmap_id}/attributes?{query_string}"
            )
        };

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let attributes: Attributes = response.json().await?;
        Ok(attributes)
    }
}
