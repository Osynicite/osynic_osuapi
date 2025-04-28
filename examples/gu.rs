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
Error: reqwest::Error: reqwest::Error {
    kind: Decode,
    source: Error("invalid type: map, expected a string", line: 1, column: 1)
}

WDF? this is not all of string?

ill adjust this soon.
*/
