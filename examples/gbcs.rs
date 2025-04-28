// get beatmap checksum
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect(
        "Please set the API_KEY environment variable to a valid osu! API v1 API key",
    );
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetBeatmapsParams::default()
        .hash("69f77b0dfe67d288c1bf748f91ceb133".to_string());

    let beatmaps = client.beatmap.get_beatmaps(params).await?;
    println!("{:?}", beatmaps);

    Ok(())
}