// Get forums
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::forum::IForum;
use osynic_osuapi::v2::model::forum::dtos::request::CreateTopicParams;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env
        ::var("ACCESS_TOKEN")
        .expect(
            "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token"
        );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });

    let params = CreateTopicParams::default()
        .title("Ciallo～(∠・ω< )⌒★".to_string())
        .body("Ciallo～(∠・ω< )⌒★".to_string())
        .forum_id(10);

    let topic = client.forum.create_topic(params).await?;
    println!("{:?}", topic);
    Ok(())
}

/*
ReqwestForum create_topic
Error: NetworkError: Response {
	url: "https://osu.ppy.sh/api/v2/forums/topics",
	status: 401,
	headers: {
		"date": "Mon, 12 May 2025 13:32:40 GMT",
		"content-type": "application/json",
		"content-length": "26",
		"connection": "keep-alive",
		"cache-control": "no-cache, private",
		"x-ratelimit-limit": "1200",
		"x-ratelimit-remaining": "1199",
		"strict-transport-security": "max-age=31536000; includeSubDomains; preload",
		"cf-cache-status": "DYNAMIC",
		"x-content-type-options": "nosniff",
		"server": "cloudflare",
		"cf-ray": "93ea4891ec9f2ddd-HKG"
	}
}
*/
