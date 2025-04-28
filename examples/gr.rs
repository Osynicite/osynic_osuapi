// Get replay
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::replay::IReplay;
use osynic_osuapi::v1::model::replay::GetReplayParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetReplayParams::default()
        .beatmap_id("3134382".to_string())
        .user("Islatri".to_string())
        .mode(2);

    let replays = client.replay.get_replay(params).await?;
    println!("{:?}", replays);

    Ok(())
}


/*
ReqwestReplay get_replay
Replay {
	content: "Something about 20.2KiB"
	encoding: "base64"
}
*/