use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::ranking::IRanking;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::ranking::enums::ranking_type::RankingType;
use crate::v2::model::ranking::structs::rankings::{KudosuRankings, Rankings};
use crate::v2::model::ranking::structs::spotlights::Spotlights;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooRanking {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IRanking for GlooRanking {
    async fn get_kudosu_ranking(&self, page: Option<u32>) -> Result<KudosuRankings> {
        console::log_1(&JsValue::from_str("GlooRanking get_kudosu_ranking"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(page) = page {
            query_params.push(("page", page.to_string()));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/rankings/kudosu")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/rankings/kudosu?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let kudosu_rankings: KudosuRankings = response.json().await?;
        Ok(kudosu_rankings)
    }

    async fn get_ranking(
        &self,
        mode: Mode,
        ranking_type: RankingType,
        country: Option<String>,
        cursor_string: Option<String>,
        filter: Option<String>,
        spotlight: Option<String>,
        variant: Option<String>,
    ) -> Result<Rankings> {
        console::log_1(&JsValue::from_str("GlooRanking get_ranking"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(country) = country {
            query_params.push(("country", country));
        }
        if let Some(cursor_string) = cursor_string {
            query_params.push(("cursor", cursor_string));
        }
        if let Some(filter) = filter {
            query_params.push(("filter", filter));
        }
        if let Some(spotlight) = spotlight {
            query_params.push(("spotlight", spotlight));
        }
        if let Some(variant) = variant {
            query_params.push(("variant", variant));
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/rankings/{mode}/{ranking_type}")
        } else {
            format!(
                "{proxy_url}https://osu.ppy.sh/api/v2/rankings/{mode}/{ranking_type}?{query_string}"
            )
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let rankings: Rankings = response.json().await?;
        Ok(rankings)
    }

    async fn get_spotlights(&self) -> Result<Spotlights> {
        console::log_1(&JsValue::from_str("GlooRanking get_spotlights"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/spotlights");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let spotlights: Spotlights = response.json().await?;
        Ok(spotlights)
    }
}
