use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::interface::beatmapsets::IBeatmapsets;


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client = OsynicOsuApiV2Client::new(OToken{
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzNjEwNCIsImp0aSI6IjYzZTg5NWMwYTY1MWIyMTA4MDI5NWY2N2FlZDk3NjdjZmMyZjFkNzExNzhlNjQ2NDE4ZWJjNjA0ZmIzZjIzYTU0OTA2MjI0MGQwZGUyODU5IiwiaWF0IjoxNzQwOTI0MTA1Ljc4MzQ2MiwibmJmIjoxNzQwOTI0MTA1Ljc4MzQ2NCwiZXhwIjoxNzQxMDEwNTA1Ljc3MzU2Nywic3ViIjoiIiwic2NvcGVzIjpbInB1YmxpYyJdfQ.J-h0I_W4inUMdq2UhYkX0hoWyD7riF-xbxPuMd6TGuFkaJs9najmEwAEJ9Uj6KI6Ac1ZbP2DAGDjH8Xs0oqcQFhKG0Z5oESW9d9BP3kKEt3NF1FTzXEV1rPX19KGpwdzQJVO_zM1dv8_6eX1_c3OG9VhgJz2j1H4MoJv4OHGSOwyi_P3VGiNGFAhE_XH8dnUYV8c1On_fBvNu-ng-c1MBJAckmZAzNvtqwqzX3PiQCpnBzSvtmUwCjzispvnKTb2yzXapVKCRRGvXfJw2KrWs3vkbSaJIL24kuaoc_gMETJaW2gQ3q9rcpuw9ApvpTKQLg4D3ApC7ckrRUlMW296NvBIPc1Ad9cxIJDZrGCx8euhfwMLpx1KtaeH8KXR4_vDIC_VtDlWRL6CjmDk6PgcutSoGoBWaPc9nN6_0wAb-cFmKbhMPV5osxi565ZpiimvwnDQCvDf3VWy1c7azKYo0tNJ5zthxTt3Bvh5qj1tC2caruelL1DAi8qxhYiiAyqQujq7jseFDLTGwWE8WiB-Kxfs_X6Edn5j_5b2uzxFwZ9kudsWkYZepm0mpH51nt7gimBSpsBKUWvyYGRW3jW5dPcVMCgKjFKsC0uL9CJV81H0XgXOrDEwShHZHRnUAXTnMTUsdUSykbvUd69N0yOJebwtkNpWVKhMEwjo7OqzdqQ".to_string(),
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    client.beatmapsets.search().await?;
    Ok(())
}