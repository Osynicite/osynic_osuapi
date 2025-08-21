use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::events::IEvents;
use crate::v2::model::event::dtos::response::GetEventsResponse;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooEvents {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IEvents for GlooEvents {
    async fn get_events(
        &self,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetEventsResponse> {
        console::log_1(&JsValue::from_str("GlooEvents get_events"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }
        if let Some(cursor_string) = cursor_string {
            query_params.push(("cursor_string", cursor_string));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/events", proxy_url)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/events?{}", proxy_url, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let events_response: GetEventsResponse = response.json().await?;
        Ok(events_response)
    }
}
