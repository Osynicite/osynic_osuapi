use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::oauth::IOauth;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooOauth {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IOauth for GlooOauth {
    async fn get_token(
        &self,
        client_id: u32,
        client_secret: String,
        code: String,
        redirect_uri: String,
    ) -> Result<OToken> {
        console::log_1(&JsValue::from_str("GlooOauth get_token"));

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [
            ("client_id", client_id.to_string()),
            ("client_secret", client_secret),
            ("code", code),
            ("grant_type", "authorization_code".to_string()),
            ("redirect_uri", redirect_uri),
        ];

        let url = format!("{}https://osu.ppy.sh/oauth/token", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&params)?)
            .send()
            .await?;

        let response = check_res(res)?;
        let token_response: OToken = response.json().await?;

        Ok(token_response)
    }

    async fn refresh_token(&self, client_id: u32, client_secret: String) -> Result<OToken> {
        console::log_1(&JsValue::from_str("GlooOauth refresh_token"));

        let refresh_token = {
            let token = self.o_token.read().await;
            token.refresh_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [
            ("client_id", client_id.to_string()),
            ("client_secret", client_secret),
            ("grant_type", "refresh_token".to_string()),
            ("refresh_token", refresh_token),
        ];

        let url = format!("{}https://osu.ppy.sh/oauth/token", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&params)?)
            .send()
            .await?;

        let response = check_res(res)?;
        let token_response: OToken = response.json().await?;

        Ok(token_response)
    }

    async fn revoke_token(&self, access_token: String) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooOauth revoke_token"));

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [("token", access_token)];

        let url = format!("{}https://osu.ppy.sh/oauth/revoke", proxy_url);

        let res = Request::delete(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&params)?)
            .send()
            .await?;

        let _response = check_res(res)?;

        Ok(())
    }
}
