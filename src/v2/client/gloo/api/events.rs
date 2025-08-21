use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::events::IEvents;
use crate::v2::model::event::structs::event::Event;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooEvents {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IEvents for GlooEvents {
    async fn get_events(&self, sort: Option<String>, cursor_string: Option<String>) -> Result<Vec<Event>> {
        console::log_1(&JsValue::from_str("GlooEvents get_events"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(sort) = sort {
            params.push(("sort", sort));
        }
        if let Some(cursor_string) = cursor_string {
            params.push(("cursor_string", cursor_string));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/events?{}",
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
        let events: Vec<Event> = response.json().await?;

        Ok(events)
    }
}
