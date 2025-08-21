use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::chat::IChat;
use crate::v2::model::chat::structs::channel::Channel;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooChat {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IChat for GlooChat {
    async fn get_updates(&self, since: Option<u32>, channel_id: Option<u32>, includes: Option<Vec<String>>) -> Result<Channel> {
        console::log_1(&JsValue::from_str("GlooChat get_updates"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(since) = since {
            params.push(("since", since.to_string()));
        }
        if let Some(channel_id) = channel_id {
            params.push(("channel_id", channel_id.to_string()));
        }
        if let Some(includes) = includes {
            for include in includes {
                params.push(("includes[]", include));
            }
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/chat/updates?{}",
            proxy_url,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let channel: Channel = response.json().await?;

        Ok(channel)
    }
}
