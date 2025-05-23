// Get forums
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::forum::IForum;
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
    let post = client
        .forum
        .reply_topic("Ciallo".to_string(), "Ciallo～(∠・ω< )⌒★".to_string())
        .await?;
    println!("{:?}", post);
    Ok(())
}

/*
ReqwestForum reply_topic
Error: NetworkError: Response {
    url: "https://osu.ppy.sh/api/v2/forums/topics/Ciallo/reply",
    status: 401,
    headers: {
        "date": "Mon, 12 May 2025 13:09:36 GMT",
        "content-type": "application/json",
        "content-length": "26",
        "connection": "keep-alive",
        "cache-control": "no-cache, private",
        "x-ratelimit-limit": "1200",
        "x-ratelimit-remaining": "1198",
        "strict-transport-security": "max-age=31536000; includeSubDomains; preload",
        "cf-cache-status": "DYNAMIC",
        "x-content-type-options": "nosniff",
        "server": "cloudflare",
        "cf-ray": "93ea26c5ad1c080f-HKG"
    }
}
*/
