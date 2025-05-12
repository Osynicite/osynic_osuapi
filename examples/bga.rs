// Beatmap Get Attributes
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::beatmaps::IBeatmaps;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
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
    let attributes = client
        .beatmaps
        .get_beatmap_attributes(5000433, None, Some(Mode::Osu), None)
        .await?;
    println!("{:?}", attributes);

    Ok(())
}
/*
ReqwestBeatmaps get_Beatmap_Attributes
Attributes {
    attributes: DifficultyAttributes {
        max_combo: 0,
        star_rating: 6.40266
    }
}
*/
