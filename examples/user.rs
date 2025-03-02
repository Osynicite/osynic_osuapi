use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::users::IUsers;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzNjEwNCIsImp0aSI6ImNlYmZlMDJjMmIxMTFlYjRkZTE2MjYyODU0OWE0OGRlOTY1ODJjYWU1NzkzYjc0ZGM5YjFhNGE4Y2UyMDg4MDkwZTYyZDMwMjlkNjQ0ODEyIiwiaWF0IjoxNzQwNDYwOTc5Ljg5OTkzMiwibmJmIjoxNzQwNDYwOTc5Ljg5OTkzNCwiZXhwIjoxNzQwNTQ3Mzc5Ljg5MjE4LCJzdWIiOiIiLCJzY29wZXMiOlsicHVibGljIl19.GlzQnANFY-5W_X9tLk-4bByK_OE0JRtRJ0YvbqpuBgIXnbAHVsX4Faidv-LK5dsZ8F5dw0pL3e-6sz_lh9E9R1U7XaGA9uMIGIcxVKT3RGv55P3Jm7zqpk2v15CO-e1Ogj-uGJug3OxW0fYvuiCSpd1fQRSIdNLCPKxI-l11nvXR7572pKN6VysI8MOIoCkAZYpz5EuUbz5n53_96GgNQlOTRfeZHoVnROoLsT606rWbIU_u95ACoaduqxAK6nscJaZWZCrGEnAjBR3zoc5BOZzqJoK37kcTryyt1dfZ_mc7cuW6pc79BMH5kPDnWSpMd73190OEWCCu50WbvaI1-WORYnVpuqQu-MgDhinXT-auvJGSQ7jNDAIyZaHZrCMa1LUBHg6zPoZ5rumPKw7cHcJ-Li6jbsqiHASNRBQWA79cnMnNiWRIbO7bjyl6MWPqQom8VWtggHVOfCRS73hJKdaFG_29j9zfqk9iGDfQ4lKwzKn7Rdr6637-A1fuhq2pDCvNaD1RHIZLo17Ei9vjE_G_goaoXOC4gM7hyIwlicndvjb-vVHM79J3RXu2wickd-vJtd9CP5iZv2uQfAswU9QvdNdXA5ztrFQk4Dft3WX91U6DddCg9DNDdmwWF91ykTjv_G3tYNRNzcmClgo3GeNew67EPu-Gt9cTRIKpp0U".to_string(),
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let user = client.users.get_user("Islatri",None).await?;
    println!("{:?}", user);
    println!("osu_account_id: {}", user.id);
    println!("username: {}", user.username);
    println!("join_date: {}", user.join_date);
    println!("country_code: {}", user.country.code);
    println!("country_name: {}", user.country.name);
    println!("cover_url: {}", user.cover_url);
    Ok(())
}