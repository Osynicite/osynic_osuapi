// Beatmapset Download
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::beatmapsets::IBeatmapsets;
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
    let beatmapsets_id = 2105932;
    client.beatmapsets.download(beatmapsets_id).await?;
    Ok(())
}

// ACG的话，就会下载下来这个
// {"authentication":"basic"}
// CCG的话，则是
/*
Error: NetworkError: Response {
    url: "https://osu.ppy.sh/api/v2/beatmapsets/2105932/download",
    status: 403,
    headers: {
        "date": "Sun, 11 May 2025 11:58:54 GMT",
        "content-type": "application/json",
        "transfer-encoding": "chunked",
        "connection": "keep-alive",
        "cache-control": "no-cache, private",
        "strict-transport-security": "max-age=31536000; includeSubDomains; preload",
        "vary": "accept-encoding",
        "cf-cache-status": "DYNAMIC",
        "x-content-type-options": "nosniff",
        "server": "cloudflare",
        "cf-ray": "93e181d58cd5e2ed-HKG"
    }
}
error: process didn 't exit successfully: `target\debug\examples\bsd.exe` (exit code: 1)
*/
