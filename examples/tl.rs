#![allow(unused)]

use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::Client;
use tokio::time::{sleep, Duration};

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
    o_token: Arc<RwLock<OToken>>,
}

impl ReqwestOauth {
    async fn update_token(&self, new_token: OToken) {
        let mut token = self.o_token.write().await;
        *token = new_token;
    }

    async fn get_token(&self) -> String {
        let token = self.o_token.read().await;
        token.value.clone()
    }
}

struct ReqwestUsers {
    client: Client,
    o_token: Arc<RwLock<OToken>>,
}

impl ReqwestUsers {
    async fn get_token(&self) -> String {
        let token = self.o_token.read().await;
        token.value.clone()
    }
}

pub struct OsynicOsuApiV2Client {
    oauth: ReqwestOauth,
    users: ReqwestUsers,
    o_token: Arc<RwLock<OToken>>,
    pub client: Client,
}

impl Default for OsynicOsuApiV2Client {
    fn default() -> Self {
        let client = Client::new();
        let o_token = Arc::new(RwLock::new(OToken::default()));
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

#[tokio::main]
async fn main() {
    // 创建默认的客户端
    let api_client = Arc::new(OsynicOsuApiV2Client::default());

    // 启动一个写入任务
    let writer_client = Arc::clone(&api_client);
    let writer_handle = tokio::spawn(async move {
        let mut version = 0;
        loop {
            // 模拟写入操作，每 5 秒更新一次 Token
            sleep(Duration::from_secs(5)).await;
            let new_token = OToken {
                value: format!("new_token_{}", version),
            };
            writer_client.oauth.update_token(new_token).await;
            version += 1;
            println!("Writer: Updated token to version {}", version);
        }
    });

    // 启动多个读取任务
    let mut reader_handles = vec![];
    for i in 1..=9 {
        let reader_client = Arc::clone(&api_client);
        let handle = tokio::spawn(async move {
            loop {
                // 模拟读取操作，每 1 秒读取一次 Token
                sleep(Duration::from_secs(1)).await;
                let oauth_token = reader_client.oauth.get_token().await;
                let users_token = reader_client.users.get_token().await;
                println!(
                    "Reader {}: OAuth Token = {:?}, Users Token = {:?}",
                    i, oauth_token, users_token
                );
            }
        });
        reader_handles.push(handle);
    }

    // 等待任务完成（实际上它们会一直运行）
    let _ = tokio::join!(writer_handle);
    for handle in reader_handles {
        let _ = handle.await;
    }
}