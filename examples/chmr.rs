// Mark channel as read
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::chat::IChat;
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
    let updates = client
        .chat
        .mark_channel_as_read(
            "Ciallo".to_string(),
            "Ciallo～(∠・ω< )⌒★".to_string(),
            "114514".to_string(),
            "1919810".to_string(),
        )
        .await?;
    println!("{:?}", updates);
    Ok(())
}

/*
ReqwestChat mark_channel_as_read
Error: NetworkError: Response {
	url: "https://osu.ppy.sh/api/v2/chat/channels/Ciallo/mark-as-read/Ciallo%EF%BD%9E(%E2%88%A0%E3%83%BB%CF%89%3C%20)%E2%8C%92%E2%98%85?channel_id=114514&message_id=1919810",
	status: 403,
	headers: {
		"date": "Mon, 12 May 2025 16:08:28 GMT",
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
		"cf-ray": "93eb2cca88010978-HKG"
	}
}
*/