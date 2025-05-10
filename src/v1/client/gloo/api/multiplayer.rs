use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct GlooMultiplayer {
    pub api_key: Arc<Mutex<String>>,
}

// 实现用户API的相关方法