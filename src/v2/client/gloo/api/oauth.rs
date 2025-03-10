use gloo_net::http::{Headers, Request, RequestBuilder};
use web_sys::{RequestCredentials, RequestInit, RequestMode};

pub struct GlooOauth {
    pub o_token: Arc<RwLock<OToken>>,
}
