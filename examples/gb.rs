// Get beatmap by hash
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set.");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetBeatmapsParams::default().hash("69f77b0dfe67d288c1bf748f91ceb133".to_string());

    let beatmaps = client.beatmap.get_beatmaps(params).await?;
    println!("{:?}", beatmaps);

    Ok(())
}

/*
ReqwestBeatmap get_beatmaps
[Beatmap {
    approved: "1",
    submit_date: "2010-01-16 04:36:21",
    approved_date: "2010-01-18 17:26:31",
    last_update: "2010-01-18 14:26:22",
    artist: "fripSide",
    beatmap_id: "46004",
    beatmapset_id: "12216",
    bpm: "139.5",
    creator: "DJPop",
    creator_id: "2363",
    difficultyrating: "2.49815",
    diff_aim: "1.14847",
    diff_speed: "1.23241",
    diff_size: "4",
    diff_overall: "4",
    diff_approach: "4",
    diff_drain: "3",
    hit_length: "81",
    source: "To Aru Kagaku no Railgun",
    genre_id: "3",
    language_id: "3",
    title: "LEVEL5 -judgelight- (TV Size)",
    total_length: "89",
    version: "Normal",
    file_md5: "69f77b0dfe67d288c1bf748f91ceb133",
    mode: "0",
    tags: "とある科学の超電磁砲",
    favourite_count: "1313",
    rating: "9.23261",
    playcount: "946634",
    passcount: "424577",
    count_normal: "75",
    count_slider: "60",
    count_spinner: "1",
    max_combo: "335",
    storyboard: "0",
    video: "1",
    download_unavailable: "0",
    audio_unavailable: "0"
}]

*/
