use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::wiki::IWiki;
use crate::v2::model::oauth::structs::o_token::OToken;
use crate::v2::model::wiki::WikiPage;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone)]
pub struct GlooWiki {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IWiki for GlooWiki {
    async fn get_wiki_page(&self, locale: String, path: String) -> Result<WikiPage> {
        console::log_1(&JsValue::from_str("GlooWiki get_wiki_page"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/wiki/{locale}/{path}");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let wiki_page: WikiPage = response.json().await?;
        Ok(wiki_page)
    }
}
