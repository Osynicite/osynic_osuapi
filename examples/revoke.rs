// Get user information by id or username
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let user = client
        .users
        .get_user_by_username("Islatri", None, None)
        .await?;
    println!("{:?}", user);

        // Revoke Token
    client
        .oauth
        .revoke_current_token()
        .await?;

    // Check if the token is revoked
    let result = client
        .users
        .get_own_data(Some(Mode::Osu), None)
        .await;
    
    match result {
        Ok(_) => println!("Token is still valid"),
        Err(e) => println!("Token is revoked: {:?}", e),
    }

    Ok(())
}
