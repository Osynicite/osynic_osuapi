// Get changelog listing
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::changelog::IChangelog;
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
    let changelog_build = client
        .changelog
        .get_changelog_build("stable40".to_string(), "20250401.2".to_string())
        .await?;

    println!("Changelog Build: {:?}", changelog_build);

    Ok(())
}

/*
ReqwestChangelog get_changelog_build
Changelog Build: ChanglogBuild {
    id: 7948,
    display_version: "20250401.2",
    users: 5913,
    created_at: "2025-04-01T12:08:29+00:00",
    update_stream: UpdateStream {
        id: 5,
        name: "stable40",
        display_name: Some("Stable"),
        is_featured: true,
        user_count: None
    },
    version: Some("20250401.2"),
    youtube_id: None,
    changelog_entries: Some([ChangelogEntry {
        id: None,
        repository: None,
        github_pull_request_id: None,
        github_url: None,
        url: Some("https://osu.ppy.sh/home/news/2025-04-01-springtime-showdown-art-contest-results"),
        entry_type: "fix",
        category: "Misc",
        title: Some("New menu artwork from latest fanart contest!"),
        message: None,
        message_html: None,
        major: true,
        created_at: Some("2025-04-01T11:45:21+00:00"),
        github_user: Some(GithubUser {
            display_name: "peppy",
            github_url: None,
            github_username: None,
            id: None,
            osu_username: Some("peppy"),
            user_id: Some(2),
            user_url: Some("https://osu.ppy.sh/users/2")
        }),
        versions: None
    }]),
    versions: Some(Versions {
        next: None,
        previous: Some(VersionsBuild {
            id: 7917,
            version: "20250309.2",
            display_version: "20250309.2",
            users: 11,
            created_at: "2025-03-08T18:01:20+00:00",
            update_stream: UpdateStream {
                id: 5,
                name: "stable40",
                display_name: Some("Stable"),
                is_featured: true,
                user_count: None
            }
        })
    })
}
*/
