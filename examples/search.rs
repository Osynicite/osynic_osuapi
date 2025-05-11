// Get search page
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::search::ISearch;
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
    let search = client
        .search
        .search(None,Some("Islatri".to_string()),Some(1))
        .await?;
    println!("{:?}", search);
    Ok(())
}

/*
ReqwestSearch search
SearchResponse {
	user: Some(UserSearchResponse {
		data: [User {
			avatar_url: "https://a.ppy.sh/31175842?1738717996.jpeg",
			country_code: "CN",
			default_group: Some("default"),
			id: 31175842,
			is_active: true,
			is_bot: false,
			is_deleted: false,
			is_online: false,
			is_supporter: false,
			last_visit: "2025-05-10T15:41:06+00:00",
			pm_friends_only: false,
			profile_colour: None,
			username: "Islatri"
		}],
		total: 1
	}),
	wiki_page: Some(WikiPageSearchResponse {
		data: [],
		total: 0
	})
}
*/