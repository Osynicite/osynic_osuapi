use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::ranking::IRanking;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::ranking::structs::rankings::Rankings;
use crate::v2::model::spotlight::structs::spotlights::Spotlights;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooRanking {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IRanking for GlooRanking {
    async fn get_ranking(
        &self,
        mode: Mode,
        r#type: String,
        country: Option<String>,
        cursor: Option<String>,
        filter: Option<String>,
        spotlight: Option<u32>,
        variant: Option<String>,
    ) -> Result<Rankings> {
        console::log_1(&JsValue::from_str("GlooRanking get_ranking"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(country) = country {
            params.push(("country", country));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        if let Some(filter) = filter {
            params.push(("filter", filter));
        }
        if let Some(spotlight) = spotlight {
            params.push(("spotlight", spotlight.to_string()));
        }
        if let Some(variant) = variant {
            params.push(("variant", variant));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/rankings/{}/{}?{}",
            proxy_url,
            mode.to_ruleset(),
            r#type,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let rankings: Rankings = response.json().await?;

        Ok(rankings)
    }

    async fn get_spotlights(&self) -> Result<Spotlights> {
        console::log_1(&JsValue::from_str("GlooRanking get_spotlights"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let url = format!("{}https://osu.ppy.sh/api/v2/spotlights", proxy_url);

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let spotlights: Spotlights = response.json().await?;

        Ok(spotlights)
    }
}
