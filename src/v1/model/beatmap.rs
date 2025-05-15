use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beatmap {
    pub approved: String, // 4 = loved, 3 = qualified, 2 = approved, 1 = ranked, 0 = pending, -1 = WIP, -2 = graveyard
    pub submit_date: String, // date submitted, in UTC
    pub approved_date: Option<String>, // date ranked, in UTC
    pub last_update: String, // last update date, in UTC. May be after approved_date if map was unranked and reranked.
    pub artist: String,
    pub beatmap_id: String,    // beatmap_id is per difficulty
    pub beatmapset_id: String, // beatmapset_id groups difficulties into a set
    pub bpm: String,
    pub creator: String,
    pub creator_id: String,
    pub difficultyrating: String, // The number of stars the map would have in-game and on the website
    pub diff_aim: Option<String>,
    pub diff_speed: Option<String>,
    pub diff_size: String,     // Circle size value (CS)
    pub diff_overall: String,  // Overall difficulty (OD)
    pub diff_approach: String, // Approach Rate (AR)
    pub diff_drain: String,    // Health drain (HP)
    pub hit_length: String,    // seconds from first note to last note not including breaks
    pub source: String,
    pub genre_id: String, // 0 = any, 1 = unspecified, 2 = video game, 3 = anime, 4 = rock, 5 = pop, 6 = other, 7 = novelty, 9 = hip hop, 10 = electronic, 11 = metal, 12 = classical, 13 = folk, 14 = jazz (note that there's no 8)
    pub language_id: String, // 0 = any, 1 = unspecified, 2 = english, 3 = japanese, 4 = chinese, 5 = instrumental, 6 = korean, 7 = french, 8 = german, 9 = swedish, 10 = spanish, 11 = italian, 12 = russian, 13 = polish, 14 = other
    pub title: String,       // song name
    pub total_length: String, // seconds from first note to last note including breaks
    pub version: String,     // difficulty name
    pub file_md5: String,    // md5 hash of the beatmap
    pub mode: String,        // game mode
    pub tags: String,        // Beatmap tags separated by spaces.
    pub favourite_count: String, // Number of times the beatmap was favourited. (Americans: notice the ou!)
    pub rating: String,
    pub packs: Option<String>, // Beatmap packs the beatmap is in. (comma separated)
    pub playcount: String,     // Number of times the beatmap was played
    pub passcount: String, // Number of times the beatmap was passed, completed (the user didn't fail or retry)
    pub count_normal: String,
    pub count_slider: String,
    pub count_spinner: String,
    pub max_combo: String, // The maximum combo a user can reach playing this beatmap.
    pub storyboard: String, // If this beatmap has a storyboard
    pub video: String,     // If this beatmap has a video
    pub download_unavailable: String, // If the download for this beatmap is unavailable (old map, etc.)
    pub audio_unavailable: String, // If the audio for this beatmap is unavailable (DMCA takedown, etc.)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBeatmapsParamsRaw {
    pub k: Option<String>,     // API key (required)
    pub since: Option<String>, // Return beatmaps ranked/loved since this date (MySQL format in UTC)
    pub s: Option<String>,     // Beatmapset ID
    pub b: Option<String>,     // Beatmap ID
    pub u: Option<String>,     // User ID or username
    pub t: Option<String>,     // Specify if u is a user_id or username ("string" or "id")
    pub m: Option<u8>,         // Game mode (0=osu!, 1=Taiko, 2=CtB, 3=mania)
    pub a: Option<u8>,         // Include converted beatmaps (0=not included, 1=included)
    pub h: Option<String>,     // Beatmap hash
    pub limit: Option<u32>,    // Amount of results (max 500)
    pub mods: Option<u32>,     // Mods that apply to the beatmap
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBeatmapsParams {
    pub api_key: Option<String>,   // API key (required)
    pub since: Option<String>, // Return beatmaps ranked/loved since this date (MySQL format in UTC)
    pub sid: Option<String>,   // Beatmapset ID
    pub bid: Option<String>,   // Beatmap ID
    pub uid: Option<String>,   // User ID or username
    pub typee: Option<String>, // Specify if u is a user_id or username ("string" or "id")
    pub mode: Option<u8>,      // Game mode (0=osu!, 1=Taiko, 2=CtB, 3=mania)
    pub has_converted: Option<u8>, // Include converted beatmaps (0=not included, 1=included)
    pub hash: Option<String>,  // Beatmap hash
    pub limit: Option<u32>,    // Amount of results (max 500)
    pub mods: Option<u32>,     // Mods that apply to the beatmap
}

impl GetBeatmapsParams {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn since(mut self, since: String) -> Self {
        self.since = Some(since);
        self
    }

    pub fn sid(mut self, sid: String) -> Self {
        self.sid = Some(sid);
        self
    }

    pub fn bid(mut self, bid: String) -> Self {
        self.bid = Some(bid);
        self
    }

    pub fn uid(mut self, uid: String) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn typee(mut self, typee: String) -> Self {
        self.typee = Some(typee);
        self
    }

    pub fn mode(mut self, mode: u8) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn has_converted(mut self, has_converted: u8) -> Self {
        self.has_converted = Some(has_converted);
        self
    }

    pub fn hash(mut self, hash: String) -> Self {
        self.hash = Some(hash);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn mods(mut self, mods: u32) -> Self {
        self.mods = Some(mods);
        self
    }

    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.api_key {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref since) = self.since {
            params.push(("since".to_string(), since.clone()));
        }
        if let Some(ref sid) = self.sid {
            params.push(("s".to_string(), sid.clone()));
        }
        if let Some(ref bid) = self.bid {
            params.push(("b".to_string(), bid.clone()));
        }
        if let Some(ref uid) = self.uid {
            params.push(("u".to_string(), uid.clone()));
        }
        if let Some(ref typee) = self.typee {
            params.push(("type".to_string(), typee.clone()));
        }
        if let Some(ref mode) = self.mode {
            params.push(("m".to_string(), mode.to_string()));
        }
        if let Some(ref has_converted) = self.has_converted {
            params.push(("a".to_string(), has_converted.to_string()));
        }
        if let Some(ref hash) = self.hash {
            params.push(("h".to_string(), hash.clone()));
        }
        if let Some(ref limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(ref mods) = self.mods {
            params.push(("mods".to_string(), mods.to_string()));
        }

        params
    }

    pub fn to_raw(&self) -> GetBeatmapsParamsRaw {
        GetBeatmapsParamsRaw {
            k: self.api_key.clone(),
            since: self.since.clone(),
            s: self.sid.clone(),
            b: self.bid.clone(),
            u: self.uid.clone(),
            t: self.typee.clone(),
            m: self.mode,
            a: self.has_converted,
            h: self.hash.clone(),
            limit: self.limit,
            mods: self.mods,
        }
    }
}

impl GetBeatmapsParamsRaw {
    pub fn k(mut self, api_key: String) -> Self {
        self.k = Some(api_key);
        self
    }

    pub fn since(mut self, since: String) -> Self {
        self.since = Some(since);
        self
    }

    pub fn s(mut self, sid: String) -> Self {
        self.s = Some(sid);
        self
    }

    pub fn b(mut self, bid: String) -> Self {
        self.b = Some(bid);
        self
    }

    pub fn u(mut self, uid: String) -> Self {
        self.u = Some(uid);
        self
    }

    pub fn t(mut self, typee: String) -> Self {
        self.t = Some(typee);
        self
    }

    pub fn m(mut self, mode: u8) -> Self {
        self.m = Some(mode);
        self
    }

    pub fn a(mut self, has_converted: u8) -> Self {
        self.a = Some(has_converted);
        self
    }

    pub fn h(mut self, hash: String) -> Self {
        self.h = Some(hash);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn mods(mut self, mods: u32) -> Self {
        self.mods = Some(mods);
        self
    }

    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.k {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref since) = self.since {
            params.push(("since".to_string(), since.clone()));
        }
        if let Some(ref sid) = self.s {
            params.push(("s".to_string(), sid.clone()));
        }
        if let Some(ref bid) = self.b {
            params.push(("b".to_string(), bid.clone()));
        }
        if let Some(ref uid) = self.u {
            params.push(("u".to_string(), uid.clone()));
        }
        if let Some(ref typee) = self.t {
            params.push(("type".to_string(), typee.clone()));
        }
        if let Some(ref mode) = self.m {
            params.push(("m".to_string(), mode.to_string()));
        }
        if let Some(ref has_converted) = self.a {
            params.push(("a".to_string(), has_converted.to_string()));
        }
        if let Some(ref hash) = self.h {
            params.push(("h".to_string(), hash.clone()));
        }
        if let Some(ref limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(ref mods) = self.mods {
            params.push(("mods".to_string(), mods.to_string()));
        }

        params
    }
}
