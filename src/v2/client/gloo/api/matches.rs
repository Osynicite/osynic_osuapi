use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::matches::IMatches;
use crate::v2::model::matches::dtos::response::{GetMatchResponse, GetMatchesListingResponse};
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooMatches {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IMatches for GlooMatches {
    async fn get_matches_listing(
        &self,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetMatchesListingResponse> {
        console::log_1(&JsValue::from_str("GlooMatches get_matches_listing"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(sort) = sort {
            query_params.push(("sort", sort));
        }
        if let Some(cursor) = cursor_string {
            query_params.push(("cursor_string", cursor));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/matches", proxy_url)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/matches?{}", proxy_url, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let matches_listing: GetMatchesListingResponse = response.json().await?;
        Ok(matches_listing)
    }

    async fn get_match(
        &self,
        match_id: u64,
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    ) -> Result<GetMatchResponse> {
        console::log_1(&JsValue::from_str("GlooMatches get_match"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(before) = before {
            query_params.push(("before", before.to_string()));
        }
        if let Some(after) = after {
            query_params.push(("after", after.to_string()));
        }
        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{}https://osu.ppy.sh/api/v2/matches/{}", proxy_url, match_id)
        } else {
            format!("{}https://osu.ppy.sh/api/v2/matches/{}?{}", proxy_url, match_id, query_string)
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let match_response: GetMatchResponse = response.json().await?;
        Ok(match_response)
    }
}
