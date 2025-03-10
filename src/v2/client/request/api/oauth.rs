use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::oauth::IOauth;
use crate::v2::model::oauth::enums::scope::Scope;
use crate::v2::model::oauth::structs::o_token::OToken;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize)]
struct GetTokenRequest {
    client_id: u64,
    client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    grant_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GetTokenResponse {
    token_type: String,
    expires_in: u64,
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
}

pub struct ReqwestOauth {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IOauth for ReqwestOauth {
    async fn get_token_with_code(
        &self,
        client_id: u64,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<OToken> {
        println!("ReqwestOauth get_token_with_code");

        let request = GetTokenRequest {
            client_id,
            client_secret: client_secret.to_string(),
            code: Some(code.to_string()),
            grant_type: "authorization_code".to_string(),
            refresh_token: None,
            redirect_uri: Some(redirect_uri.to_string()),
            scope: None,
        };

        println!("{:?}", request);

        // let params = [
        //     ("client_id", client_id.to_string()),
        //     ("client_secret", client_secret.to_string()),
        //     ("code", code.to_string()),
        //     ("grant_type", "authorization_code".to_string()),
        //     ("redirect_uri", redirect_uri.to_string()),
        // ];

        // println!("{:?}", params);

        let response = self
            .client
            .post("https://osu.ppy.sh/oauth/token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&request)
            .send()
            .await?;

        println!("{:?}", response);

        // 解析响应
        let token_response: GetTokenResponse = response.json().await?;

        // 将 GetTokenResponse 转换为 OToken
        let o_token = OToken {
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
        };

        {
            let mut token = self.o_token.write().await;
            *token = o_token.clone();
        }

        Ok(o_token)
    }

    async fn get_token_without_code(&self, client_id: u64, client_secret: &str) -> Result<OToken> {
        println!("ReqwestOauth get_token_without_code");

        let request = GetTokenRequest {
            client_id,
            client_secret: client_secret.to_string(),
            code: None,
            grant_type: "client_credentials".to_string(),
            refresh_token: None,
            redirect_uri: None,
            scope: Some(Scope::default().to_string()),
        };

        let res = self
            .client
            .post("https://osu.ppy.sh/oauth/token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&request)
            .send()
            .await?;

        let response = check_res(res)?;

        let token_response: GetTokenResponse = response.json().await?;

        let o_token = OToken {
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            access_token: token_response.access_token,
            refresh_token: None,
        };

        Ok(o_token)
    }

    async fn refresh_token(
        &self,
        client_id: u64,
        client_secret: &str,
        scope: Option<Vec<Scope>>,
    ) -> Result<OToken> {
        println!("ReqwestOauth refresh_token");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };
        let refresh_token = {
            let token = self.o_token.read().await;
            token.refresh_token.clone()
        };

        if refresh_token.is_none() {
            return Err("No refresh token, Please check your authorization type."
                .to_string()
                .into());
        }

        let request = GetTokenRequest {
            client_id,
            client_secret: client_secret.to_string(),
            code: None,
            refresh_token,
            grant_type: "refresh_token".to_string(),
            redirect_uri: None,
            scope: scope.map(|scopes| {
                scopes
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("+")
            }),
        };

        let res = self
            .client
            .post("https://osu.ppy.sh/oauth/token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&request)
            .send()
            .await?;

        let response = check_res(res)?;

        let token_response: GetTokenResponse = response.json().await?;

        let o_token = OToken {
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
        };

        {
            let mut token = self.o_token.write().await;
            *token = o_token.clone();
        }

        Ok(o_token)
    }

    async fn revoke_current_token(&self) -> Result<()> {
        println!("ReqwestOauth revoke_current_token");

        let access_token = &self.o_token.read().await.access_token;

        // 创建 DELETE 请求
        let response = self
            .client
            .delete("https://osu.ppy.sh/api/v2/oauth/tokens/current")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        // 检查响应状态码
        if response.status() == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            // 如果状态码不是 204，返回错误
            Err("Failed to revoke current token".to_string().into())
        }
    }
}
