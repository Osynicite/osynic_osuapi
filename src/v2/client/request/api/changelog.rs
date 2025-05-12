use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::changelog::IChangelog;
use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::changelog::ChangelogListing;
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestChangelog {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IChangelog for ReqwestChangelog {
    async fn get_changelog_build(&self, stream: String, build: String) -> Result<ChanglogBuild> {
        println!("ReqwestChangelog get_changelog_build");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get(format!(
                "https://osu.ppy.sh/api/v2/changelog/{}/{}",
                stream, build
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let response = check_res(res)?;

        let changelog_build: ChanglogBuild = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let Changelog: GetChangelogResponse = serde_json::from_str(&text)?;

        Ok(changelog_build)
    }
    async fn get_changelog_listing(
        &self,
        from: Option<String>,
        max_id: Option<u32>,
        stream: Option<String>,
        to: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> Result<ChangelogListing> {
        println!("ReqwestChangelog get_changelog_listing");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/changelog")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("from", from.map(|s| s.to_string())),
                ("max_id", max_id.map(|s| s.to_string())),
                ("stream", stream.map(|s| s.to_string())),
                ("to", to.map(|s| s.to_string())),
                (
                    "message_formats",
                    message_formats
                        .map(|v| v.join(","))
                        .or(Some("html,markdown".to_string())),
                ),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let changelog_listing: ChangelogListing = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let matchh: GetMatchResponse = serde_json::from_str(&text)?;

        Ok(changelog_listing)
    }
    async fn lookup_changelog_build(
        &self,
        changelog: String,
        key: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> Result<ChanglogBuild> {
        println!("ReqwestChangelog lookup_changelog_build");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        // message_formats 如果为空，则默认填入html,markdown

        let res = self
            .client
            .get(format!("https://osu.ppy.sh/api/v2/changelog/{}", changelog))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("key", key.map(|x| x.to_string())),
                (
                    "message_formats",
                    message_formats
                        .map(|v| v.join(","))
                        .or(Some("html,markdown".to_string())),
                ),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let changelog_build: ChanglogBuild = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let Changelog: GetChangelogResponse = serde_json::from_str(&text)?;

        Ok(changelog_build)
    }
}
