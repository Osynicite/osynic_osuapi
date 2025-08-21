use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::search::ISearch;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::search::dtos::response::SearchResponse;
use crate::v2::model::search::enums::search_mode::SearchMode;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooSearch {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl ISearch for GlooSearch {
    async fn search(
        &self,
        mode: Option<SearchMode>,
        query: Option<String>,
        page: Option<u32>,
    ) -> Result<SearchResponse> {
        console::log_1(&JsValue::from_str("GlooSearch search"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(mode) = mode {
            query_params.push(("mode", mode.to_string()));
        }
        if let Some(query) = query {
            query_params.push(("query", query));
        }
        if let Some(page) = page {
            query_params.push(("page", page.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/search")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/search?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let search_response: SearchResponse = response.json().await?;
        Ok(search_response)
    }
}
