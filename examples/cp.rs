// Get comments
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::comments::IComments;
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
    let comment = client
        .comments
        .post_comment(None, None, None, Some("Ciallo～(∠・ω< )⌒★".to_string()))
        .await?;
    println!("{:?}", comment);
    Ok(())
}

/*
ReqwestComments post_comment
Error: NetworkError: Response {
    url: "https://osu.ppy.sh/api/v2/comments?comment.message=Ciallo%EF%BD%9E%28%E2%88%A0%E3%83%BB%CF%89%3C+%29%E2%8C%92%E2%98%85",
    status: 403,
    headers: {
        "date": "Mon, 12 May 2025 13:02:01 GMT",
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
        "cf-ray": "93ea1baccf22e903-HKG"
    }
}
*/
