use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchMode {
    #[serde(rename = "all")]
    #[default]
    All,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "wiki_page")]
    WikiPage,
}

impl SearchMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchMode::All => "All",
            SearchMode::User => "User",
            SearchMode::WikiPage => "WikiPage",
        }
    }
    pub fn to_search_param(&self) -> String {
        match self {
            SearchMode::All => "all".to_string(),
            SearchMode::User => "user".to_string(),
            SearchMode::WikiPage => "wiki_page".to_string(),
        }
    }
}

impl std::fmt::Display for SearchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
