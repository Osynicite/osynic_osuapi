use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct OsynicOsuApiV2GlooClient {
    pub o_token: Arc<Mutex<OToken>>,
}

impl OsynicOsuApiV1GlooClient {
    pub fn new(o_token: String) -> Self {
        let o_token = Arc::new(Mutex::new(o_token));
        OsynicOsuApiV1GlooClient {
            o_token,
        }
    }

    pub fn set_o_token(&self, o_token: String) {
        let mut token = self.o_token.lock().unwrap();
        *token = o_token;
    }
}

impl Default for OsynicOsuApiV1GlooClient {
    fn default() -> Self {
        let o_token = Arc::new(Mutex::new(String::new()));
        OsynicOsuApiV1GlooClient {
            o_token,
        }
    }
}
