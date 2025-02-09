use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    let client = OsynicOsuApiV2Client::default();
    let token = client.oauth.get_token_without_code(client_id.parse().unwrap(), &client_secret).await?;
    println!("{:?}", token);
    Ok(())
}