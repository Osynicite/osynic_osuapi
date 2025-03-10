use osynic_osuapi::error::Result;
use osynic_osuapi::utils::country_code_to_unicode_flag;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // ACG can't get me
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzNjEwNCIsImp0aSI6IjA0OTI2ZGJjN2Q1YmEzNWZmZTNiOTlkODQ5MjkwZWJiNTVlZGJmODcwYjI2YmQ5NDI0Y2UwZmVmNmRiNGEzM2VkZjJiNmMwNWVjZjY3YmUxIiwiaWF0IjoxNzQwNDUzMTkyLjkxODY0NywibmJmIjoxNzQwNDUzMTkyLjkxODY0OSwiZXhwIjoxNzQwNTM5NTkyLjkwOTQ0MSwic3ViIjoiIiwic2NvcGVzIjpbInB1YmxpYyJdfQ.L1o7nTTkZivbJicj0IxgzPgk4DapbZ2GVu5TC61kuVNtHoPl7gAtT7xOyd47VkQKVD54rsPeUn7HfEWF1gHlm3F5bqpEK_2JJxwObYezYLitK2jubcsHLmHIsL0TQYUooX2fSi0AqdSnjP55lfpDVUo4gyfRiQnMUcysUdFpKQ28p4hV6b-ZM4AaSOeR_lU7MYNjfR2UcufR8pPQ1gcJ8IymBKs_2NDS0ouEbgm2YKyxfl8mIb3MUcPCeybDrO-3MoadpvgJlCbluTfI8vwLSoSWRTaDvPumBb1h4R8nQfCI1zJmQjPbe6wt7bbgAET80hEZeINTBxwZgKzSSwYZAfVW_TwoVsEjispYZlXiON9L3bxFsXT1ZBDEBITG00A6KIF5dT2ylhc0pGAvM-IOqQ_OSrd3O4WIJxAjUoddSyIHkhdbV9zFjlYZ-O-6-xzWT3UIoB_hpnd8ykbQ-jweACmg0D3e0Ha0pMDFnz0gvFRQNwvImc0jObmSDqKojVMpcB0wXac8cX9-wWGB0Q3C9Y4zTNeKT_gEC3ijp3h9EJLMB1tvteq8jzEi5J0iJwCe7cNqHP8lBSq7kR7VtX3wqlPMpkpstAogFtxb8vfL1MVFvSoGrdjxa-XOlcKSqlnguNoQy0blN6WLKddz_PsUof2mxI1STgawqSYCPI5g6gs".to_string(),
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
