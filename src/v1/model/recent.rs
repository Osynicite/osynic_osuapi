use serde::{Deserialize, Serialize};

/// 用户最近游玩记录结构体
/// User recent plays structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentPlay {
    pub beatmap_id: String,   // 谱面ID
    pub score: String,        // 得分
    pub maxcombo: String,     // 最大连击
    pub count50: String,      // 50分击打数量
    pub count100: String,     // 100分击打数量
    pub count300: String,     // 300分击打数量
    pub countmiss: String,    // 未击中数量
    pub countkatu: String,    // katu计数（优良）
    pub countgeki: String,    // geki计数（激）
    pub perfect: String,      // 1 = 达到谱面最大连击数, 0 = 未达到
    pub enabled_mods: String, // 使用的mod的位运算标志表示
    pub user_id: String,      // 用户ID
    pub date: String,         // 日期，UTC格式
    pub rank: String,         // 评级（SS, S, A等）
}

/// 获取用户最近游玩记录的原始参数
/// Raw parameters for getting user's recent plays
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserRecentParamsRaw {
    pub k: Option<String>,  // API密钥（必需）
    pub u: Option<String>,  // 用户ID或用户名（必需）
    pub m: Option<u8>,      // 游戏模式（0=osu!，1=Taiko，2=CtB，3=osu!mania）。可选，默认值为0
    pub limit: Option<u32>, // 返回结果数量（范围在1到50之间 - 默认为10）
    pub t: Option<String>, // 指定u是用户ID还是用户名。使用"string"表示用户名，"id"表示用户ID。可选，默认为自动识别
}

/// 获取用户最近游玩记录的参数
/// Parameters for getting user's recent plays
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserRecentParams {
    pub api_key: Option<String>, // API密钥（必需）
    pub user: Option<String>,    // 用户ID或用户名（必需）
    pub mode: Option<u8>,        // 游戏模式（0=osu!，1=Taiko，2=CtB，3=osu!mania）。可选，默认值为0
    pub limit: Option<u32>,      // 返回结果数量（范围在1到50之间 - 默认为10）
    pub typee: Option<String>, // 指定user是用户ID还是用户名。使用"string"表示用户名，"id"表示用户ID。可选，默认为自动识别
}

impl GetUserRecentParams {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
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

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn typee(mut self, typee: String) -> Self {
        self.typee = Some(typee);
        self
    }

    /// 构建参数列表
    /// Build parameter list
    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.api_key {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref user) = self.user {
            params.push(("u".to_string(), user.clone()));
        }
        if let Some(ref mode) = self.mode {
            params.push(("m".to_string(), mode.to_string()));
        }
        if let Some(ref limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(ref typee) = self.typee {
            params.push(("type".to_string(), typee.clone()));
        }

        params
    }

    /// 转换为原始参数
    /// Convert to raw parameters
    pub fn to_raw(&self) -> GetUserRecentParamsRaw {
        GetUserRecentParamsRaw {
            k: self.api_key.clone(),
            u: self.user.clone(),
            m: self.mode.clone(),
            limit: self.limit.clone(),
            t: self.typee.clone(),
        }
    }
}

impl GetUserRecentParamsRaw {
    pub fn k(mut self, api_key: String) -> Self {
        self.k = Some(api_key);
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

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn t(mut self, typee: String) -> Self {
        self.t = Some(typee);
        self
    }

    /// 构建参数列表
    /// Build parameter list
    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.k {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref user) = self.u {
            params.push(("u".to_string(), user.clone()));
        }
        if let Some(ref mode) = self.m {
            params.push(("m".to_string(), mode.to_string()));
        }
        if let Some(ref limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(ref typee) = self.t {
            params.push(("type".to_string(), typee.clone()));
        }

        params
    }
}
