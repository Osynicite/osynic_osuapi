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
        .lookup_changelog_build(
			"2025.511.0".to_string(),
			None,
			None,
		)
        .await?;
	
	println!("Changelog Build: {:?}", changelog_build);
	

    Ok(())
}

/*
ReqwestChangelog lookup_changelog_build
Changelog Build: ChanglogBuild {
	id: 7987,
	display_version: "2025.511.0",
	users: 0,
	created_at: "2025-05-11T15:13:26+00:00",
	update_stream: UpdateStream {
		id: 8,
		name: "web",
		display_name: Some("Web"),
		is_featured: false,
		user_count: None
	},
	version: Some("2025.511.0"),
	youtube_id: None,
	changelog_entries: Some([ChangelogEntry {
		id: Some(30338),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13228),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13228"),
		url: None,
		entry_type: "fix",
		category: "Wiki",
		title: Some("[EN/ZH] Cleanup `People/Community contributors`"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-08T09:24:17+00:00"),
		github_user: Some(GithubUser {
			display_name: "YumeMuzi",
			github_url: Some("https://github.com/YumeMuzi"),
			github_username: Some("YumeMuzi"),
			id: Some(587),
			osu_username: Some("Muziyami"),
			user_id: Some(7003013),
			user_url: Some("https://osu.ppy.sh/users/7003013")
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30339),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13172),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13172"),
		url: None,
		entry_type: "fix",
		category: "Wiki",
		title: Some("Update `Bespoke music`"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-08T12:46:31+00:00"),
		github_user: Some(GithubUser {
			display_name: "Walavouchey",
			github_url: Some("https://github.com/Walavouchey"),
			github_username: Some("Walavouchey"),
			id: Some(317),
			osu_username: Some("Walavouchey"),
			user_id: Some(5773079),
			user_url: Some("https://osu.ppy.sh/users/5773079")
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30341),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13234),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13234"),
		url: None,
		entry_type: "fix",
		category: "Wiki",
		title: Some("Change the mention of \"rhythm game\" to \"game\" in `Difficulty naming schemes`"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-09T04:11:01+00:00"),
		github_user: Some(GithubUser {
			display_name: "HannahBruch",
			github_url: Some("https://github.com/HannahBruch"),
			github_username: Some("HannahBruch"),
			id: Some(1109),
			osu_username: None,
			user_id: None,
			user_url: None
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30347),
		repository: Some("ppy/osu-web"),
		github_pull_request_id: Some(12160),
		github_url: Some("https://github.com/ppy/osu-web/pull/12160"),
		url: None,
		entry_type: "fix",
		category: "Api",
		title: Some("Fix cursor for topic listing not working"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-09T10:11:32+00:00"),
		github_user: Some(GithubUser {
			display_name: "nanaya",
			github_url: Some("https://github.com/nanaya"),
			github_username: Some("nanaya"),
			id: Some(2),
			osu_username: Some("nanaya"),
			user_id: Some(2387883),
			user_url: Some("https://osu.ppy.sh/users/2387883")
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30348),
		repository: Some("ppy/osu-web"),
		github_pull_request_id: Some(12162),
		github_url: Some("https://github.com/ppy/osu-web/pull/12162"),
		url: None,
		entry_type: "fix",
		category: "Search",
		title: Some("Adjust menu search button"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-09T12:34:02+00:00"),
		github_user: Some(GithubUser {
			display_name: "nanaya",
			github_url: Some("https://github.com/nanaya"),
			github_username: Some("nanaya"),
			id: Some(2),
			osu_username: Some("nanaya"),
			user_id: Some(2387883),
			user_url: Some("https://osu.ppy.sh/users/2387883")
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30351),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13236),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13236"),
		url: None,
		entry_type: "add",
		category: "Wiki",
		title: Some("Add `osu!catch World Cup 2025: Registrations Now Open!`, `CWC 2025` wiki article"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-09T15:04:15+00:00"),
		github_user: Some(GithubUser {
			display_name: "LeoFLT",
			github_url: Some("https://github.com/LeoFLT"),
			github_username: Some("LeoFLT"),
			id: Some(556),
			osu_username: Some("LeoFLT"),
			user_id: Some(3668779),
			user_url: Some("https://osu.ppy.sh/users/3668779")
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30352),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13226),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13226"),
		url: None,
		entry_type: "add",
		category: "Wiki",
		title: Some("Add `Squad Global Taiko Showdown 2025: Registrations Now Open!` news post"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-09T21:55:22+00:00"),
		github_user: Some(GithubUser {
			display_name: "Kasumisama",
			github_url: Some("https://github.com/Kasumisama"),
			github_username: Some("Kasumisama"),
			id: Some(578),
			osu_username: Some("Kasumi-sama"),
			user_id: Some(6177263),
			user_url: Some("https://osu.ppy.sh/users/6177263")
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30354),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13238),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13238"),
		url: None,
		entry_type: "add",
		category: "Wiki",
		title: Some("Add `Resurrection Cup 2025: Registrations Now Open!` news post"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-10T21:22:27+00:00"),
		github_user: Some(GithubUser {
			display_name: "Albionthegreat",
			github_url: Some("https://github.com/Albionthegreat"),
			github_username: Some("Albionthegreat"),
			id: Some(644),
			osu_username: None,
			user_id: None,
			user_url: None
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30355),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13240),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13240"),
		url: None,
		entry_type: "fix",
		category: "Wiki",
		title: Some("Fix link in `Resurrection Cup 2025: Registrations Now Open!`"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-10T21:35:19+00:00"),
		github_user: Some(GithubUser {
			display_name: "Albionthegreat",
			github_url: Some("https://github.com/Albionthegreat"),
			github_username: Some("Albionthegreat"),
			id: Some(644),
			osu_username: None,
			user_id: None,
			user_url: None
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30356),
		repository: Some("ppy/osu-wiki"),
		github_pull_request_id: Some(13235),
		github_url: Some("https://github.com/ppy/osu-wiki/pull/13235"),
		url: None,
		entry_type: "fix",
		category: "Wiki",
		title: Some("Update `Gulano Cup #5` tournament article"),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-11T10:58:04+00:00"),
		github_user: Some(GithubUser {
			display_name: "Reihynn",
			github_url: Some("https://github.com/Reihynn"),
			github_username: Some("Reihynn"),
			id: Some(1092),
			osu_username: None,
			user_id: None,
			user_url: None
		}),
		versions: None
	}, ChangelogEntry {
		id: Some(30358),
		repository: Some("ppy/osu-web"),
		github_pull_request_id: Some(12164),
		github_url: Some("https://github.com/ppy/osu-web/pull/12164"),
		url: None,
		entry_type: "fix",
		category: "Misc",
		title: Some("Fix unlocalised Chat channel text in message mail notification "),
		message: Some(""),
		message_html: None,
		major: false,
		created_at: Some("2025-05-11T15:01:06+00:00"),
		github_user: Some(GithubUser {
			display_name: "notbakaneko",
			github_url: Some("https://github.com/notbakaneko"),
			github_username: Some("notbakaneko"),
			id: Some(1),
			osu_username: Some("notbakaneko"),
			user_id: Some(10751776),
			user_url: Some("https://osu.ppy.sh/users/10751776")
		}),
		versions: None
	}]),
	versions: Some(Versions {
		next: None,
		previous: Some(VersionsBuild {
			id: 7986,
			version: "2025.508.0",
			display_version: "2025.508.0",
			users: 0,
			created_at: "2025-05-08T09:20:19+00:00",
			update_stream: UpdateStream {
				id: 8,
				name: "web",
				display_name: Some("Web"),
				is_featured: false,
				user_count: None
			}
		})
	})
}
*/
