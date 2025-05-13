use serde::{Deserialize, Serialize};

/// 回放数据结构体
/// Replay data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replay {
    pub content: String,  // base64编码的回放数据（LZMA流）
    pub encoding: String, // 编码方式，通常为"base64"
}

/// 获取回放数据的原始参数
/// Raw parameters for getting replay data
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReplayParamsRaw {
    pub k: Option<String>, // API密钥（必需）
    pub b: Option<String>, // 谱面ID（不是谱面集ID！）（必需，除非指定了s参数）
    pub u: Option<String>, // 游玩该谱面的用户（必需，除非指定了s参数）
    pub m: Option<u8>,     // 游玩的模式
    pub s: Option<String>, // 指定要获取回放数据的分数ID，可以代替b和u参数
    pub t: Option<String>, // 指定u是用户ID还是用户名。使用"string"表示用户名，"id"表示用户ID。可选，默认为自动识别
    pub mods: Option<u32>, // 指定mod或mod组合（见位运算枚举）
}

/// 获取回放数据的参数
/// Parameters for getting replay data
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReplayParams {
    pub api_key: Option<String>,    // API密钥（必需）
    pub beatmap_id: Option<String>, // 谱面ID（不是谱面集ID！）（必需，除非指定了score_id参数）
    pub user: Option<String>,       // 游玩该谱面的用户（必需，除非指定了score_id参数）
    pub mode: Option<u8>,           // 游玩的模式
    pub score_id: Option<String>,   // 指定要获取回放数据的分数ID，可以代替beatmap_id和user参数
    pub typee: Option<String>, // 指定user是用户ID还是用户名。使用"string"表示用户名，"id"表示用户ID。可选，默认为自动识别
    pub mods: Option<u32>,     // 指定mod或mod组合（见位运算枚举）
}

impl GetReplayParams {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn beatmap_id(mut self, beatmap_id: String) -> Self {
        self.beatmap_id = Some(beatmap_id);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn mode(mut self, mode: u8) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn score_id(mut self, score_id: String) -> Self {
        self.score_id = Some(score_id);
        self
    }

    pub fn typee(mut self, typee: String) -> Self {
        self.typee = Some(typee);
        self
    }

    pub fn mods(mut self, mods: u32) -> Self {
        self.mods = Some(mods);
        self
    }

    /// 构建参数列表
    /// Build parameter list
    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.api_key {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref beatmap_id) = self.beatmap_id {
            params.push(("b".to_string(), beatmap_id.clone()));
        }
        if let Some(ref user) = self.user {
            params.push(("u".to_string(), user.clone()));
        }
        if let Some(ref mode) = self.mode {
            params.push(("m".to_string(), mode.to_string()));
        }
        if let Some(ref score_id) = self.score_id {
            params.push(("s".to_string(), score_id.clone()));
        }
        if let Some(ref typee) = self.typee {
            params.push(("type".to_string(), typee.clone()));
        }
        if let Some(ref mods) = self.mods {
            params.push(("mods".to_string(), mods.to_string()));
        }

        params
    }

    /// 转换为原始参数
    /// Convert to raw parameters
    pub fn to_raw(&self) -> GetReplayParamsRaw {
        GetReplayParamsRaw {
            k: self.api_key.clone(),
            b: self.beatmap_id.clone(),
            u: self.user.clone(),
            m: self.mode,
            s: self.score_id.clone(),
            t: self.typee.clone(),
            mods: self.mods,
        }
    }
}

impl GetReplayParamsRaw {
    pub fn k(mut self, api_key: String) -> Self {
        self.k = Some(api_key);
        self
    }

    pub fn b(mut self, beatmap_id: String) -> Self {
        self.b = Some(beatmap_id);
        self
    }

    pub fn u(mut self, user: String) -> Self {
        self.u = Some(user);
        self
    }

    pub fn m(mut self, mode: u8) -> Self {
        self.m = Some(mode);
        self
    }

    pub fn s(mut self, score_id: String) -> Self {
        self.s = Some(score_id);
        self
    }

    pub fn t(mut self, typee: String) -> Self {
        self.t = Some(typee);
        self
    }

    pub fn mods(mut self, mods: u32) -> Self {
        self.mods = Some(mods);
        self
    }

    /// 构建参数列表
    /// Build parameter list
    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.k {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref beatmap_id) = self.b {
            params.push(("b".to_string(), beatmap_id.clone()));
        }
        if let Some(ref user) = self.u {
            params.push(("u".to_string(), user.clone()));
        }
        if let Some(ref mode) = self.m {
            params.push(("m".to_string(), mode.to_string()));
        }
        if let Some(ref score_id) = self.s {
            params.push(("s".to_string(), score_id.clone()));
        }
        if let Some(ref typee) = self.t {
            params.push(("type".to_string(), typee.clone()));
        }
        if let Some(ref mods) = self.mods {
            params.push(("mods".to_string(), mods.to_string()));
        }

        params
    }
}
