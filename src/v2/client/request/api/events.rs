use crate::error::Result;
use crate::v2::client::request::check::check_res;
use crate::v2::interface::events::IEvents;
use crate::v2::model::event::dtos::response::GetEventsResponse;
use crate::v2::model::oauth::structs::o_token::OToken;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ReqwestEvents {
    pub client: reqwest::Client,
    pub o_token: Arc<RwLock<OToken>>,
}

impl IEvents for ReqwestEvents {
    async fn get_events(
        &self,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetEventsResponse> {
        println!("ReqwestEvents get_events");

        let access_token = {
            let token = self.o_token.read().await;
            token.access_token.clone()
        };

        let res = self
            .client
            .get("https://osu.ppy.sh/api/v2/events")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("sort", sort.map(|s| s.to_string())),
                ("cursor", cursor_string.map(|s| s.to_string())),
            ])
            .send()
            .await?;

        let response = check_res(res)?;

        let events: GetEventsResponse = response.json().await?;

        // let text = response.text().await?;
        // println!("Response text: {}", text);
        // let events: GetEventsResponse = serde_json::from_str(&text)?;

        Ok(events)
    }
}
