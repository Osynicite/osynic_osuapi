use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::utils::country_code_to_unicode_flag;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzNjEwNCIsImp0aSI6ImRkYzFmOGE0NTE3NzFiMGRjYjNjM2MxYjExNjhiYTllNTdhMmI1OGMzMzVjODYxN2M2OGVhNzRmZmI3OTdhMDZjZjBmZmE4NzNmNjQ4MWRkIiwiaWF0IjoxNzM5NzA4NTQyLjM1MjYxOCwibmJmIjoxNzM5NzA4NTQyLjM1MjYyMSwiZXhwIjoxNzM5Nzk0OTQyLjM0NDEyMywic3ViIjoiIiwic2NvcGVzIjpbInB1YmxpYyJdfQ.fjyuloLIkx5J9W3w34IkgUKd_RqgzDnv77x36xF-_wMQF_B2l67VnFa77_2brIBYMBQe5yjjJszsSZtnPr78n0PiFZbIXIvjA0ntW_az0f8cGS_wHYHPqv-vatPeUnwQulF1nndxMH8sfio9rTDg1PHMti8u_0UN25nr1cIsPCz4DxF7W7SIR0oNLT_kkOsrgwY-LiOLWSKLiLKRCE0lzl4ahKBLY2eRiiQg5DdHW0gFhralA8hKXBVqChGX0lYjYIU9JYfFM3X8k54MCvwCJSP5Op85EmXK5VngdVkrFvtjYBOUjEluOB_MXzOvb_qlmjBX4m6ZlQSHntqonOgzjaCAbVozz9YZEa56hXss8ggGAXvjTO3EbLDPIU-WFRTZ-hjTzGD1toG2WiWDmTmS7FrXyA6Thvo8XxKrTNtGF8UsmUWzkJ4uGR0Gj7AvmGyIrPmhNOfhgaoMzAFTu2olWj-Xmm4ib3iiYLzeY_PVz1eMg2VDpwkSkrmMtOZJTcm-gyHbTC0sxtdUDRkvBpmB6CrnYwTIB2AiUx1zo1rCAIidUCM-5iYtHRlMq_ynxYpi2ttum4SFuJtwmCWubfBCkF4Xew3CeRkoKUu33jWsyJg6ulZcZAuCGKKb0Okj5QJTYSTzWOt-Ebobajt9AseyixUaOf3zKbjVKuZ3LAghdUQ".to_string(),
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let me = client.users.get_own_data(None).await?;
    println!("{:?}", me);
    println!("osu_account_id: {}", me.id);
    println!("username: {}", me.username);
    // println!("join_date: {}", me.join_date);
    // println!("country_name: {}", me.country.name);
    // println!("country_code: {}", me.country.code);
    let country_flag = country_code_to_unicode_flag(&me.country.code).unwrap_or_default();
    // println!("country_flag: {}", country_flag);
    let country_svg_path = format!("https://osu.ppy.sh/assets/images/flags/{}.svg", country_flag);
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