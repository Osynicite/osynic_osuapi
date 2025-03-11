use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
// use osynic_osuapi::v2::model::oauth::enums::scope::Scope;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let scopes = vec![Scope::Public, Scope::Identify];

    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let refresh_token = std::env::var("REFRESH_TOKEN").expect(
        "Please set the REFRESH_TOKEN environment variable to a valid osu! API v2 refresh token",
    );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: Some(refresh_token),
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let token = client
        .oauth
        .refresh_token(client_id.parse().unwrap(), &client_secret, None)
        .await?;
    println!("{:?}", token);
    let me = client.users.get_own_data(Some(Mode::Osu),None).await?;
    println!("{:?}", me);
    Ok(())
}
