// Get user
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::user::GetUserParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetUserParams::default().user("Islatri".to_string());

    let users = client.user.get_user(params).await?;
    println!("{:?}", users);

    Ok(())
}

/*
ReqwestUser get_user
[User {
    user_id: "31175842",
    username: "Islatri",
    join_date: "2022-08-30 06:57:47",
    count300: "676156",
    count100: "136433",
    count50: "22732",
    playcount: "4966",
    ranked_score: "625695631",
    total_score: "2413072414",
    pp_rank: "0",
    level: "71.5153",
    pp_raw: "0",
    accuracy: "90.64440131187439",
    count_rank_ss: "2",
    count_rank_ssh: "1",
    count_rank_s: "47",
    count_rank_sh: "1",
    count_rank_a: "112",
    country: "CN",
    total_seconds_played: "321075",
    pp_country_rank: "60272",
    events: []
}]
*/
