use reqwest::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};
use std::fs::File;
use std::io::copy;
use osynic_osuapi::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 设置请求的URL
    let url = "https://osu.ppy.sh/api/v2/beatmapsets/114514/download";

    // 创建客户端
    let client = reqwest::Client::new();

    // 发送GET请求
    let response = client
        .get(url)
        .header(AUTHORIZATION, "Bearer YOUR_ACCESS_TOKEN_HERE")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    // 检查响应状态
    if response.status().is_success() {
        // 获取文件名（从Content-Disposition头中提取，如果没有则使用默认名称）
        let filename = "114514_beatmapset.osz";

        // 创建文件并保存内容
        let mut file = File::create(filename)?;
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut file)?;

        println!("File downloaded successfully: {}", filename);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}