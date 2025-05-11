use serde::{Deserialize, Serialize};

// 基本事件字段
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseEvent {
    pub created_at: String,
    #[serde(rename = "createdAt")]
    pub created_at_alt: String,
    pub id: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub user: User,
}

// 主事件枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Rank {
        #[serde(flatten)]
        base: BaseEvent,
        #[serde(rename = "scoreRank")]
        score_rank: String,
        rank: u32,
        mode: String,
        beatmap: Beatmap,
    },
    Achievement {
        #[serde(flatten)]
        base: BaseEvent,
        achievement: Achievement,
    },
    BeatmapsetUpdate {
        #[serde(flatten)]
        base: BaseEvent,
        beatmapset: Beatmapset,
    },
    UserSupportAgain {
        #[serde(flatten)]
        base: BaseEvent,
    },
    // 添加其他类型...
    
    // 捕获所有其他未知格式
    Unknown(BaseEvent),
}

// 在代码中处理 Event 枚举时，可以根据 base.event_type 来区分类型
impl Event {
    pub fn get_type(&self) -> &str {
        match self {
            Event::Rank { base, .. } => &base.event_type,
            Event::Achievement { base, .. } => &base.event_type,
            Event::BeatmapsetUpdate { base, .. } => &base.event_type,
            Event::UserSupportAgain { base } => &base.event_type,
            Event::Unknown(base) => &base.event_type,
        }
    }
}

// 辅助结构体定义...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmap {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Achievement {
    pub icon_url: String,
    pub id: u64,
    pub name: String,
    pub grouping: String,
    pub ordering: u32,
    pub slug: String,
    pub description: String,
    pub mode: Option<String>,
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmapset {
    pub title: String,
    pub url: String,
}