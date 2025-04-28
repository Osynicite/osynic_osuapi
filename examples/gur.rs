// Get user recent
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::recent::GetUserRecentParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetUserRecentParams::default().user("Islatri".to_string());

    let recents = client.user.get_user_recent(params).await?;
    println!("{:?}", recents);

    Ok(())
}

/*
ReqwestUserRecent get_user_recent
[]

oh its a long time ago
*/
