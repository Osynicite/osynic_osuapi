use serde::{Deserialize, Serialize};

/// 多人游戏玩家分数结构体
/// Multiplayer player score structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerScore {
    pub slot: String,                 // 玩家槽位（基于0的索引）
    pub team: String,                 // 如果模式不支持队伍，则为0；否则1=蓝队，2=红队
    pub user_id: String,              // 用户ID
    pub score: String,                // 得分
    pub maxcombo: String,             // 最大连击
    pub rank: String,                 // 未使用
    pub count50: String,              // 50分击打数量
    pub count100: String,             // 100分击打数量
    pub count300: String,             // 300分击打数量
    pub countmiss: String,            // 未击中数量
    pub countgeki: String,            // geki计数（激）
    pub countkatu: String,            // katu计数（优良）
    pub perfect: String,              // 全连击（1=是，0=否）
    pub pass: String,                 // 如果玩家在谱面结束时失败，则为0；否则（通过或复活）为1
    pub enabled_mods: Option<String>, // 启用的mod，可能为null
}

/// 多人游戏单局游戏结构体
/// Multiplayer game structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerGame {
    pub game_id: String,               // 游戏ID
    pub start_time: String,            // 开始时间，UTC格式
    pub end_time: String,              // 结束时间，UTC格式
    pub beatmap_id: String,            // 谱面ID
    pub play_mode: String,             // 游戏模式：标准=0，太鼓=1，接水果=2，音乐盒=3
    pub match_type: String,            // 未找到具体含义
    pub scoring_type: String,          // 胜利条件：分数=0，准确度=1，连击=2，分数v2=3
    pub team_type: String,             // 团队类型：单打=0，Tag合作=1，团队对抗=2，Tag团队对抗=3
    pub mods: String,                  // 全局mod，见参考
    pub scores: Vec<MultiplayerScore>, // 玩家分数列表
}

/// 多人游戏匹配结构体
/// Multiplayer match structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerMatch {
    pub match_id: String,         // 匹配ID
    pub name: String,             // 房间名称
    pub start_time: String,       // 开始时间，UTC格式
    pub end_time: Option<String>, // 结束时间，如果未结束则为null；解散时间为UTC格式
}

/// 多人游戏响应结构体
/// Multiplayer response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerResponse {
    #[serde(rename = "match")]
    pub matchh: MultiplayerMatch, // 匹配信息
    pub games: Vec<MultiplayerGame>, // 游戏列表
}

/// 获取多人游戏信息的原始参数
/// Raw parameters for getting multiplayer match information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMatchParamsRaw {
    pub k: Option<String>,  // API密钥（必需）
    pub mp: Option<String>, // 要获取信息的匹配ID（必需）
}

/// 获取多人游戏信息的参数
/// Parameters for getting multiplayer match information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMatchParams {
    pub api_key: Option<String>,  // API密钥（必需）
    pub match_id: Option<String>, // 要获取信息的匹配ID（必需）
}

impl GetMatchParams {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn match_id(mut self, match_id: String) -> Self {
        self.match_id = Some(match_id);
        self
    }

    /// 构建参数列表
    /// Build parameter list
    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.api_key {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref match_id) = self.match_id {
            params.push(("mp".to_string(), match_id.clone()));
        }

        params
    }

    /// 转换为原始参数
    /// Convert to raw parameters
    pub fn to_raw(&self) -> GetMatchParamsRaw {
        GetMatchParamsRaw {
            k: self.api_key.clone(),
            mp: self.match_id.clone(),
        }
    }
}

impl GetMatchParamsRaw {
    pub fn k(mut self, api_key: String) -> Self {
        self.k = Some(api_key);
        self
    }

    pub fn mp(mut self, match_id: String) -> Self {
        self.mp = Some(match_id);
        self
    }

    /// 构建参数列表
    /// Build parameter list
    pub fn build_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref api_key) = self.k {
            params.push(("k".to_string(), api_key.clone()));
        }
        if let Some(ref match_id) = self.mp {
            params.push(("mp".to_string(), match_id.clone()));
        }

        params
    }
}
