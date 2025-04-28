use osynic_osuapi::error::Result;
use osynic_osuapi::utils::country_code_to_unicode_flag;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // ACG can't get me
    let access_token = std::env::var("ACCESS_TOKEN").expect(
        "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token",
    );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let me = client.users.get_own_data(None, None).await?;
    println!("{:?}", me);
    println!("osu_account_id: {}", me.id);
    println!("username: {}", me.username);
    // println!("join_date: {}", me.join_date);
    // println!("country_name: {}", me.country.name);
    // println!("country_code: {}", me.country.code);
    let country_flag = country_code_to_unicode_flag(&me.country.code).unwrap_or_default();
    // println!("country_flag: {}", country_flag);
    let country_svg_path = format!(
        "https://osu.ppy.sh/assets/images/flags/{}.svg",
        country_flag
    );
    println!("country_svg_path: {}", country_svg_path);
    // playmode
    println!("playmode: {:?}", me.playmode);
    // // statistics
    // println!("statistics: {:?}", me.statistics);
    // // statistics_rulesets
    // println!("statistics_rulesets: {:?}", me.statistics_rulesets);

    // acc
    println!("acc: {:?}", me.statistics.hit_accuracy);
    // level
    println!("level: {:?}", me.statistics.level.current);
    // progress
    println!("progress: {:?}", me.statistics.level.progress);
    // global_rank
    println!("global_rank: {:?}", me.statistics.global_rank);
    // country_rank
    println!("country_rank: {:?}", me.statistics.country_rank);
    // pp
    println!("pp: {:?}", me.statistics.pp);
    println!("avatar_url: {}", me.avatar_url);
    println!("cover_url: {}", me.cover_url);
    Ok(())
}
