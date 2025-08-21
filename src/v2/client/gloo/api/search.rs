use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::search::ISearch;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::search::structs::search_result::SearchResult;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooSearch {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl ISearch for GlooSearch {
    async fn search_all(&self, query: String, page: Option<i32>) -> Result<SearchResult> {
        console::log_1(&JsValue::from_str("GlooSearch search_all"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = vec![("query", query)];
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/search?{}",
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
        let search_result: SearchResult = response.json().await?;

        Ok(search_result)
    }
}
