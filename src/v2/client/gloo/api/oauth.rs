use gloo_net::http::{Request,Headers, RequestBuilder};
use web_sys::{RequestInit, RequestMode, RequestCredentials};

pub struct GlooOauth {
    pub o_token: Arc<RwLock<OToken>>,
}

