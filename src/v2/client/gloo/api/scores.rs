use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::scores::IScores;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::score::dtos::response::GetScoresResponse;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooScores {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IScores for GlooScores {
    async fn get_scores(
        &self,
        ruleset: Option<Mode>,
        cursor_string: Option<String>,
    ) -> Result<GetScoresResponse> {
        console::log_1(&JsValue::from_str("GlooScores get_scores"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(ruleset) = ruleset {
            query_params.push(("ruleset", ruleset.to_string()));
        }
        if let Some(cursor_string) = cursor_string {
            query_params.push(("cursor_string", cursor_string));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/scores")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/scores?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let scores_response: GetScoresResponse = response.json().await?;
        Ok(scores_response)
    }
}
