use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::oauth::IOauth;
use crate::v2::model::oauth::enums::scope::Scope;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooOauth {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IOauth for GlooOauth {
    async fn get_token_with_code(
        &self,
        client_id: u64,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<OToken> {
        console::log_1(&JsValue::from_str("GlooOauth get_token_with_code"));

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = format!(
            "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
            client_id, client_secret, code, redirect_uri
        );

        let url = format!("{}https://osu.ppy.sh/oauth/token", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)?
            .send()
            .await?;

        let response = check_res(res)?;
        let token: OToken = response.json().await?;
        
        // Update stored token
        {
            let mut stored_token = self.o_token.lock().unwrap();
            *stored_token = token.clone();
        }
        
        Ok(token)
    }

    async fn get_token_without_code(
        &self,
        client_id: u64,
        client_secret: &str,
    ) -> Result<OToken> {
        console::log_1(&JsValue::from_str("GlooOauth get_token_without_code"));

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let form_data = format!(
            "client_id={}&client_secret={}&grant_type=client_credentials&scope=public",
            client_id, client_secret
        );

        let url = format!("{}https://osu.ppy.sh/oauth/token", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)?
            .send()
            .await?;

        let response = check_res(res)?;
        let token: OToken = response.json().await?;
        
        // Update stored token
        {
            let mut stored_token = self.o_token.lock().unwrap();
            *stored_token = token.clone();
        }
        
        Ok(token)
    }

    async fn refresh_token(
        &self,
        client_id: u64,
        client_secret: &str,
        scope: Option<Vec<Scope>>,
    ) -> Result<OToken> {
        console::log_1(&JsValue::from_str("GlooOauth refresh_token"));

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let refresh_token = {
            let token = self.o_token.lock().unwrap();
            token.refresh_token.clone().unwrap_or_default()
        };

        let scope_str = scope
            .map(|scopes| {
                scopes.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_else(|| "public".to_string());

        let form_data = format!(
            "client_id={}&client_secret={}&grant_type=refresh_token&refresh_token={}&scope={}",
            client_id, client_secret, refresh_token, scope_str
        );

        let url = format!("{}https://osu.ppy.sh/oauth/token", proxy_url);

        let res = Request::post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)?
            .send()
            .await?;

        let response = check_res(res)?;
        let token: OToken = response.json().await?;
        
        // Update stored token
        {
            let mut stored_token = self.o_token.lock().unwrap();
            *stored_token = token.clone();
        }
        
        Ok(token)
    }

    async fn revoke_current_token(&self) -> Result<()> {
        console::log_1(&JsValue::from_str("GlooOauth revoke_current_token"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{}https://osu.ppy.sh/api/v2/oauth/tokens/current", proxy_url);

        let res = Request::delete(&url)
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        check_res(res)?;
        Ok(())
    }
}
