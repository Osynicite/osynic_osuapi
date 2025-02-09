use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
// use osynic_osuapi::v2::model::oauth::enums::scope::Scope;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let scopes = vec![Scope::Public, Scope::Identify];
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzNjEwNCIsImp0aSI6ImM1OTZlNDI5ZWUyNDIzZDQxYjQxMTIyMDQ4ZjlmZWIwMTVlNjI0MWEyZTU0NmQ5NTNlMDQ1ZWQwOTQxNTlhN2VkNTQ5ODdmODc3OWUxMmI3IiwiaWF0IjoxNzM3NjA2ODk1LjU2NDc2MywibmJmIjoxNzM3NjA2ODk1LjU2NDc2NSwiZXhwIjoxNzM3NjkzMjk1LjU1MDkwNiwic3ViIjoiMzExNzU4NDIiLCJzY29wZXMiOlsiaWRlbnRpZnkiLCJwdWJsaWMiXX0.d1FIysOD5OiPFlfM7AiZtPZKYWO6KE6WMEn_RyxfGdibfpQh3DFGsLWHOs7BwG2P5KjQlJanBSet3UBGNOusTmX89HGVjutXURmwxBzak68n88ULLKn3iDPQSRsgg885WLTR1hTx8Xjc9KGkN0uuyR2aI0glii44dwm2B6kjD3541xFMzgFeybOZLHN7ejMMY7KfB3Z39N3cxS8ul75QZzEi30_7lmwzrWmatQ9DjlOKPJ1L3cDjLwYfdv3McAMl9gt4-X53t-I05BRC1r-4oMYw34SzKle8esBH_1PGYEiwy7P6_4yujNqAzXJrHVKnMRyOIza-aM8CewMDdMBRcbZMvw_hdovpHE3Ppl1CKLlW70ssb73eMvIC3lNpHU0p2_aZarMqN06bjSXNacpgXZS5pIcDbMg3LbjQU4rFKxSEyTvkSbKsbypwez-mA7mX6n6RWcPNPvpTJnZyKlxddrEktLX19_Z1nXupY0IMRFPC3uVdb7YYn9wbax7E2ML2nUXMlYEqPe_yqP8RKLEEIJPKfvBJVIXn03SZH1glbO7P2EVJTGjwd4f1p_-a206RzxNOinniaA8mAU3Qsgq0sIq8uR8r0Hz0q4HieFMkCUaD-INUXS3Sf64Mev2EN2wgV8LPYy9YnnbQ239Pdna2ZNW5zhUiJUhDRQ-DzdKiaHM".to_string(),
        refresh_token: Some("def50200df3b08c013dc02e06df0d17d570a1c348149368740be4451932155d3a8493f64ec44a29e66ff774bf3c3f1e803543238ded201173fc452f3c388fbce39c262fc8c817a984f82daf34daa2fa1aa622dce410ae0ade2b2bac571679f138f66363c61696b6f89b67c20253854055d04dd355f93b70456b9715e6926c00356b2ccdccd138cc677cc93b86d1b0f7bc087b01307c192e6b43e28118a50a0b752cab39897b94151262595f516307b4adcbb9a2997a78ba96b7bfc0c21473d0e8b62b6a776fe765077d81f52c42ac3ec79c8174b9b5d1f914dcd6909f5c473491dbeb19a68dd1705da5b0ee0141cb6e6033a1bad68e7027a5acc700d8d58e801f3ced514d36190f3e26ddbe5bc2955167213b164a2867d36c9ea593cf23f628f40c016e8393f05fc7523dfa453853efb0e52332e8cd33517266f2eb0da496425bc50e57b09d02c40c8afc922b4c27f95265ebd3a5712df229cca6a9aae18ce5a58673258e181e9255702f3d48aabfa796d02080dcf91e107b11aa25e7b88e7".to_string()),
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let token = client.oauth.refresh_token(client_id.parse().unwrap(), &client_secret, None).await?;
    println!("{:?}", token);
    let me = client.users.get_own_data(Some(Mode::Osu)).await?;
    println!("{:?}", me);
    Ok(())
}