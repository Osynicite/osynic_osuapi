#![allow(unused)]

use std::sync::{Arc, Mutex};
use reqwest::Client;

#[derive(Clone, Debug)]
struct OToken {
    value: String,
}

impl Default for OToken {
    fn default() -> Self {
        OToken {
            value: "default_token".to_string(),
        }
    }
}

struct ReqwestOauth {
    client: Client,
    o_token: Arc<Mutex<OToken>>,
}

impl ReqwestOauth {
    fn update_token(&self, new_token: OToken) {
        let mut token = self.o_token.lock().unwrap();
        *token = new_token;
    }
}

struct ReqwestUsers {
    client: Client,
    o_token: Arc<Mutex<OToken>>,
}

impl ReqwestUsers {
    fn print_token(&self) {
        let token = self.o_token.lock().unwrap();
        println!("Users Token: {:?}", token.value);
    }
}

pub struct OsynicOsuApiV2Client {
    oauth: ReqwestOauth,
    users: ReqwestUsers,
    o_token: Arc<Mutex<OToken>>,
    pub client: Client,
}

impl Default for OsynicOsuApiV2Client {
    fn default() -> Self {
        let client = Client::new();
        let o_token = Arc::new(Mutex::new(OToken::default()));
        OsynicOsuApiV2Client {
            o_token: o_token.clone(),
            oauth: ReqwestOauth {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            users: ReqwestUsers {
                client: client.clone(),
                o_token: o_token.clone(),
            },
            client,
        }
    }
}

fn main() {
    // 创建默认的客户端
    let api_client = OsynicOsuApiV2Client::default();

    // 打印初始 Token
    println!(
        "Initial Token: {:?}",
        api_client.o_token.lock().unwrap().value
    );

    // 更新 Token
    let new_token = OToken {
        value: "new_token_from_network".to_string(),
    };
    api_client.oauth.update_token(new_token);

    // 打印更新后的 Token
    println!(
        "Updated Token: {:?}",
        api_client.o_token.lock().unwrap().value
    );

    // 验证 users 中的 Token 是否同步更新
    api_client.users.print_token();
}