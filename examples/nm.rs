// Get notifications
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::notifications::INotifications;
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
    client
        .notifications
        .mark_notifications_as_read(None)
        .await?;
    Ok(())
}

/*
ReqwestNotifications mark_notifications_as_read
Error: NetworkError: Response {
	url: "https://osu.ppy.sh/api/v2/notifications/mark-read",
	status: 403,
	headers: {
		"date": "Sun, 11 May 2025 16:08:54 GMT",
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
		"cf-ray": "93e2f00c5f9409e4-HKG"
	}
}
*/