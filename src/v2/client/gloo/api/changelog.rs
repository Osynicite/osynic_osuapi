use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::changelog::IChangelog;
use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::changelog::ChangelogListing;
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooChangelog {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IChangelog for GlooChangelog {
    async fn get_changelog_build(&self, stream: String, build: String) -> Result<ChanglogBuild> {
        console::log_1(&JsValue::from_str("GlooChangelog get_changelog_build"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/changelog/{stream}/{build}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let build: ChanglogBuild = response.json().await?;
        Ok(build)
    }

    async fn get_changelog_listing(
        &self,
        from: Option<String>,
        max_id: Option<u32>,
        stream: Option<String>,
        to: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> Result<ChangelogListing> {
        console::log_1(&JsValue::from_str("GlooChangelog get_changelog_listing"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(from) = from {
            query_params.push(("from", from));
        }
        if let Some(max_id) = max_id {
            query_params.push(("max_id", max_id.to_string()));
        }
        if let Some(stream) = stream {
            query_params.push(("stream", stream));
        }
        if let Some(to) = to {
            query_params.push(("to", to));
        }
        if let Some(message_formats) = message_formats {
            for format in message_formats {
                query_params.push(("message_formats[]", format));
            }
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/changelog")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/changelog?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let changelog_listing: ChangelogListing = response.json().await?;
        Ok(changelog_listing)
    }

    async fn lookup_changelog_build(
        &self,
        changelog: String,
        key: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> Result<ChanglogBuild> {
        console::log_1(&JsValue::from_str("GlooChangelog lookup_changelog_build"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let mut query_params = Vec::new();
        if let Some(key) = key {
            query_params.push(("key", key));
        }
        if let Some(message_formats) = message_formats {
            for format in message_formats {
                query_params.push(("message_formats[]", format));
            }
        }

        let query_string = serde_urlencoded::to_string(&query_params)?;
        let url = if query_string.is_empty() {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/changelog/{changelog}")
        } else {
            format!("{proxy_url}https://osu.ppy.sh/api/v2/changelog/{changelog}?{query_string}")
        };

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let build: ChanglogBuild = response.json().await?;
        Ok(build)
    }
}
