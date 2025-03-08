use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::users::IUsers;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let user = client.users.get_user_by_username("Islatri",None,None).await?;
    println!("{:?}", user);
    println!("osu_account_id: {}", user.id);
    println!("username: {}", user.username);
    println!("join_date: {}", user.join_date);
    println!("country_code: {}", user.country.code);
    println!("country_name: {}", user.country.name);
    println!("cover_url: {}", user.cover_url);
    Ok(())
}