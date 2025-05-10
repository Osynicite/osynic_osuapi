// Get match
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::multiplayer::IMultiplayer;
use osynic_osuapi::v1::model::multiplayer::GetMatchParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetMatchParams::default().match_id("ciallo".to_string());

    let multiplayers = client.multiplayer.get_match(params).await?;
    println!("{:?}", multiplayers);

    Ok(())
}

/*
ReqwestMultiplayer get_match
Text: "{\"match\":0,\"games\":[]}"
MultiplayerResponse {
	matchh: MultiplayerMatch {
		match_id: "",
		name: "",
		start_time: "",
		end_time: None
	},
	games: []
}
*/
