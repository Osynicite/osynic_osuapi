use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::changelog::IChangelog;
use crate::v2::model::changelog::structs::build::Build;
use crate::v2::model::changelog::structs::changelog::Changelog;
use crate::v2::model::changelog::structs::stream::UpdateStream;
use crate::v2::model::oauth::structs::o_token::OToken;

use gloo_net::http::Request;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooChangelog {
    pub o_token: Arc<RwLock<OToken>>,
    pub proxy_url: Arc<RwLock<String>>,
}

impl IChangelog for GlooChangelog {
    async fn get_changelog_build(&self, stream: String, build: String) -> Result<Build> {
        console::log_1(&JsValue::from_str("GlooChangelog get_changelog_build"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/changelog/{}/{}",
            proxy_url, stream, build
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let build_response: Build = response.json().await?;

        Ok(build_response)
    }

    async fn get_changelog_listing(
        &self,
        from: Option<String>,
        to: Option<String>,
        max_id: Option<u32>,
        stream: Option<String>,
    ) -> Result<Changelog> {
        console::log_1(&JsValue::from_str("GlooChangelog get_changelog_listing"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let mut params = Vec::new();
        if let Some(from) = from {
            params.push(("from", from));
        }
        if let Some(to) = to {
            params.push(("to", to));
        }
        if let Some(max_id) = max_id {
            params.push(("max_id", max_id.to_string()));
        }
        if let Some(stream) = stream {
            params.push(("stream", stream));
        }

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/changelog?{}",
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
        let changelog_response: Changelog = response.json().await?;

        Ok(changelog_response)
    }

    async fn lookup_changelog_build(&self, changelog: String, key: Option<String>) -> Result<Build> {
        console::log_1(&JsValue::from_str("GlooChangelog lookup_changelog_build"));

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.read().await;
            url.clone()
        };

        let params = [("key", key.map(|x| x.to_string()))];

        let url = format!(
            "{}https://osu.ppy.sh/api/v2/changelog/{}?{}",
            proxy_url,
            changelog,
            serde_urlencoded::to_string(&params)?
        );

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;
        let build_response: Build = response.json().await?;

        Ok(build_response)
    }
}
