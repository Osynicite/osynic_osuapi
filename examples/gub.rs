// Get user best
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::best::GetUserBestParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetUserBestParams::default().user("Islatri".to_string());

    let bests = client.user.get_user_best(params).await?;
    println!("{:?}", bests);

    Ok(())
}

/*
ReqwestUserBest get_user_best
[BestScore {
    beatmap_id: "1988753",
    score_id: "4543356050",
    score: "641148",
    maxcombo: "186",
    count50: "8",
    count100: "42",
    count300: "176",
    countmiss: "8",
    countkatu: "19",
    countgeki: "27",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2023-11-27 14:54:46",
    rank: "C",
    pp: "67.0596",
    replay_available: "0"
}, BestScore {
    beatmap_id: "1988750",
    score_id: "4543319826",
    score: "1383620",
    maxcombo: "297",
    count50: "0",
    count100: "12",
    count300: "200",
    countmiss: "3",
    countkatu: "9",
    countgeki: "46",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2023-11-27 13:26:02",
    rank: "A",
    pp: "63.5653",
    replay_available: "0"
}, BestScore {
    beatmap_id: "217253",
    score_id: "4648125530",
    score: "1967590",
    maxcombo: "219",
    count50: "2",
    count100: "45",
    count300: "364",
    countmiss: "4",
    countkatu: "21",
    countgeki: "53",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2024-07-01 08:20:21",
    rank: "B",
    pp: "62.8416",
    replay_available: "0"
}, BestScore {
    beatmap_id: "1988751",
    score_id: "4539694222",
    score: "1573918",
    maxcombo: "325",
    count50: "0",
    count100: "4",
    count300: "189",
    countmiss: "0",
    countkatu: "4",
    countgeki: "53",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2023-11-20 04:56:51",
    rank: "S",
    pp: "59.967",
    replay_available: "0"
}, BestScore {
    beatmap_id: "1988748",
    score_id: "4543355424",
    score: "465200",
    maxcombo: "133",
    count50: "4",
    count100: "50",
    count300: "180",
    countmiss: "5",
    countkatu: "26",
    countgeki: "27",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2023-11-27 14:53:17",
    rank: "C",
    pp: "58.407",
    replay_available: "0"
}, BestScore {
    beatmap_id: "1613232",
    score_id: "4605386148",
    score: "367568",
    maxcombo: "137",
    count50: "2",
    count100: "25",
    count300: "128",
    countmiss: "3",
    countkatu: "11",
    countgeki: "22",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2024-03-28 16:12:00",
    rank: "B",
    pp: "53.8961",
    replay_available: "0"
}, BestScore {
    beatmap_id: "827932",
    score_id: "4472322890",
    score: "2106022",
    maxcombo: "340",
    count50: "0",
    count100: "18",
    count300: "307",
    countmiss: "1",
    countkatu: "12",
    countgeki: "65",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2023-07-14 01:38:11",
    rank: "A",
    pp: "53.0132",
    replay_available: "0"
}, BestScore {
    beatmap_id: "3853596",
    score_id: "4567316511",
    score: "5584388",
    maxcombo: "488",
    count50: "2",
    count100: "61",
    count300: "734",
    countmiss: "8",
    countkatu: "41",
    countgeki: "138",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2024-01-12 15:21:45",
    rank: "A",
    pp: "50.3696",
    replay_available: "0"
}, BestScore {
    beatmap_id: "1406451",
    score_id: "4567226857",
    score: "3221722",
    maxcombo: "430",
    count50: "1",
    count100: "40",
    count300: "363",
    countmiss: "1",
    countkatu: "26",
    countgeki: "61",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2024-01-12 11:54:16",
    rank: "B",
    pp: "50.1756",
    replay_available: "0"
}, BestScore {
    beatmap_id: "3939073",
    score_id: "4471431310",
    score: "5132088",
    maxcombo: "438",
    count50: "6",
    count100: "75",
    count300: "798",
    countmiss: "10",
    countkatu: "54",
    countgeki: "137",
    perfect: "0",
    enabled_mods: "0",
    user_id: "31175842",
    date: "2023-07-12 13:23:19",
    rank: "B",
    pp: "49.9405",
    replay_available: "0"
}]
*/
