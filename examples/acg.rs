// Authorization Code Grant
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let redirect_uri = std::env::var("REDIRECT_URI").expect("REDIRECT_URI not set");
    let code = std::env::var("CODE").expect("CODE not set");
    let client = OsynicOsuApiV2Client::default();
    let token = client
        .oauth
        .get_token_with_code(client_id.parse()?, &client_secret, &code, &redirect_uri)
        .await?;
    println!("{:?}", token);
    Ok(())
}
