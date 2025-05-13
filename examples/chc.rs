// Create channel
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::chat::IChat;
use osynic_osuapi::v2::model::chat::dtos::request::CreateChannelParams;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let params = CreateChannelParams::default()
        .channel_name("Ciallo".to_string())
        .message("Ciallo～(∠・ω< )⌒★".to_string())
        .target_id(35628968);
    let channel = client.chat.create_channel(params).await?;
    println!("{:?}", channel);
    Ok(())
}

/*
ReqwestChat create_channel
Error: NetworkError: Response {
    url: "https://osu.ppy.sh/api/v2/chat/channels",
    status: 403,
    headers: {
        "date": "Mon, 12 May 2025 16:12:24 GMT",
        "content-type": "application/json",
        "transfer-encoding": "chunked",
        "connection": "keep-alive",
        "cache-control": "no-cache, private",
        "x-ratelimit-limit": "1200",
        "x-ratelimit-remaining": "1199",
        "strict-transport-security": "max-age=31536000; includeSubDomains; preload",
        "vary": "accept-encoding",
        "cf-cache-status": "DYNAMIC",
        "x-content-type-options": "nosniff",
        "server": "cloudflare",
        "cf-ray": "93eb328e798185d4-HKG"
    }
}
*/
