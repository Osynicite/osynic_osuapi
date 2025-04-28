use serde::{Deserialize, Serialize};

/// 用户事件结构体
/// User event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    pub display_html: String,  // HTML 显示内容
    pub beatmap_id: String,    // 谱面ID
    pub beatmapset_id: String, // 谱面集ID
    pub date: String,          // 事件日期，UTC格式
    pub epicfactor: String,    // 事件的"史诗"系数（1-32之间）
}

/// 用户信息结构体
/// User information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,              // 用户ID
    pub username: String,             // 用户名
    pub join_date: String,            // 加入日期，UTC格式
    pub count300: String,             // 所有已排名、已批准和已喜爱谱面的300分击打总数
    pub count100: String,             // 所有已排名、已批准和已喜爱谱面的100分击打总数
    pub count50: String,              // 所有已排名、已批准和已喜爱谱面的50分击打总数
    pub playcount: String,            // 游玩次数，仅计算已排名、已批准和已喜爱的谱面
    pub ranked_score: String,         // 排名分数，计算每个已排名、已批准和已喜爱谱面的最佳个人分数
    pub total_score: String,          // 总分数，计算已排名、已批准和已喜爱谱面的每个分数
    pub pp_rank: String,              // PP排名
    pub level: String,                // 等级
    pub pp_raw: String,               // 原始PP值，对于不活跃玩家，该值为0以将其从排行榜中清除
    pub accuracy: String,             // 准确度
    pub count_rank_ss: String,        // SS评分数量
    pub count_rank_ssh: String,       // SSH评分数量
    pub count_rank_s: String,         // S评分数量
    pub count_rank_sh: String,        // SH评分数量
    pub count_rank_a: String,         // A评分数量
    pub country: String,              // 国家代码，使用ISO3166-1 alpha-2国家代码命名
    pub total_seconds_played: String, // 总游玩时间（秒）
    pub pp_country_rank: String,      // 国家内PP排名
    pub events: Vec<UserEvent>,       // 该用户的事件列表
}

/// 获取用户信息的原始参数
/// Raw parameters for getting user information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserParamsRaw {
    pub k: Option<String>,      // API密钥（必需）
    pub u: Option<String>,      // 用户ID或用户名（必需）
    pub m: Option<u8>,          // 游戏模式（0=osu!，1=Taiko，2=CtB，3=osu!mania）。可选，默认值为0
    pub t: Option<String>, // 指定u是用户ID还是用户名。使用"string"表示用户名，"id"表示用户ID。可选，默认为自动识别
    pub event_days: Option<u8>, // 现在与最后事件日期之间的最大天数。范围为1-31。可选，默认值为1
}

/// 获取用户信息的参数
/// Parameters for getting user information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserParams {
    pub api_key: Option<String>, // API密钥（必需）
    pub user: Option<String>,    // 用户ID或用户名（必需）
    pub mode: Option<u8>,        // 游戏模式（0=osu!，1=Taiko，2=CtB，3=osu!mania）。可选，默认值为0
    pub typee: Option<String>, // 指定user是用户ID还是用户名。使用"string"表示用户名，"id"表示用户ID。可选，默认为自动识别
    pub event_days: Option<u8>, // 现在与最后事件日期之间的最大天数。范围为1-31。可选，默认值为1
}

impl GetUserParams {
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

    pub fn typee(mut self, typee: String) -> Self {
        self.typee = Some(typee);
        self
    }

    pub fn event_days(mut self, event_days: u8) -> Self {
        self.event_days = Some(event_days);
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
        if let Some(ref typee) = self.typee {
            params.push(("type".to_string(), typee.clone()));
        }
        if let Some(ref event_days) = self.event_days {
            params.push(("event_days".to_string(), event_days.to_string()));
        }

        params
    }

    /// 转换为原始参数
    /// Convert to raw parameters
    pub fn to_raw(&self) -> GetUserParamsRaw {
        GetUserParamsRaw {
            k: self.api_key.clone(),
            u: self.user.clone(),
            m: self.mode.clone(),
            t: self.typee.clone(),
            event_days: self.event_days.clone(),
        }
    }
}

impl GetUserParamsRaw {
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

    pub fn t(mut self, typee: String) -> Self {
        self.t = Some(typee);
        self
    }

    pub fn event_days(mut self, event_days: u8) -> Self {
        self.event_days = Some(event_days);
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
        if let Some(ref typee) = self.t {
            params.push(("type".to_string(), typee.clone()));
        }
        if let Some(ref event_days) = self.event_days {
            params.push(("event_days".to_string(), event_days.to_string()));
        }

        params
    }
}
