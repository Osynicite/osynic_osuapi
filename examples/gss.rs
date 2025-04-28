// Get scores
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::scores::IScores;
use osynic_osuapi::v1::model::scores::GetScoresParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetScoresParams::default()
        .beatmap_id("3134382".to_string())
        .user("Islatri".to_string())
        .mode(2);

    let scores = client.score.get_scores(params).await?;
    println!("{:?}", scores);

    Ok(())
}

/*
ReqwestScore get_scores

[Score {
    score_id: "219374634",
    score: "1557194",
    username: "Islatri",
    count300: "320",
    count100: "4",
    count50: "271",
    countmiss: "1",
    maxcombo: "230",
    countkatu: "6",
    countgeki: "53",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2024-12-29 13:27:41",
    rank: "S",
    pp: "36.7791",
    replay_available: "1"
}]

*/
