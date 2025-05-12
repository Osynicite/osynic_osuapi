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
    let changelog_listing = client
        .changelog
        .get_changelog_listing(None, None, None, None, None)
        .await?;
    for (i, stream) in changelog_listing.streams.iter().enumerate().take(5) {
        println!("{} Streams: {:?}", i + 1, stream);
    }

    for (i, build) in changelog_listing.builds.iter().enumerate().take(5) {
        println!("{} Builds: {:?}", i + 1, build);
    }

    println!("Search: {:?}", changelog_listing.search);

    Ok(())
}

/*
ReqwestChangelog get_changelog_listing
1 Streams: Stream {
    id: 5,
    name: "stable40",
    display_name: Some("Stable"),
    is_featured: true,
    latest_build: Some(LatestBuild {
        id: 7948,
        version: "20250401.2",
        display_version: "20250401.2",
        users: 5931,
        created_at: "2025-04-01T12:08:29+00:00",
        update_stream: UpdateStream {
            id: 5,
            name: "stable40",
            display_name: Some("Stable"),
            is_featured: true,
            user_count: None
        }
    }),
    user_count: Some(5956)
}
2 Streams: Stream {
    id: 6,
    name: "beta40",
    display_name: Some("Beta"),
    is_featured: false,
    latest_build: Some(LatestBuild {
        id: 7947,
        version: "20250401.1",
        display_version: "20250401.1",
        users: 44,
        created_at: "2025-04-01T12:06:42+00:00",
        update_stream: UpdateStream {
            id: 6,
            name: "beta40",
            display_name: Some("Beta"),
            is_featured: false,
            user_count: None
        }
    }),
    user_count: Some(45)
}
3 Streams: Stream {
    id: 3,
    name: "cuttingedge",
    display_name: Some("Cutting Edge"),
    is_featured: false,
    latest_build: Some(LatestBuild {
        id: 7946,
        version: "20250401",
        display_version: "20250401",
        users: 241,
        created_at: "2025-04-01T11:53:31+00:00",
        update_stream: UpdateStream {
            id: 3,
            name: "cuttingedge",
            display_name: Some("Cutting Edge"),
            is_featured: false,
            user_count: None
        }
    }),
    user_count: Some(242)
}
4 Streams: Stream {
    id: 7,
    name: "lazer",
    display_name: Some("Lazer"),
    is_featured: false,
    latest_build: Some(LatestBuild {
        id: 7982,
        version: "2025.424.0",
        display_version: "2025.424.0",
        users: 1483,
        created_at: "2025-04-24T07:06:36+00:00",
        update_stream: UpdateStream {
            id: 7,
            name: "lazer",
            display_name: Some("Lazer"),
            is_featured: false,
            user_count: None
        }
    }),
    user_count: Some(1530)
}
5 Streams: Stream {
    id: 8,
    name: "web",
    display_name: Some("Web"),
    is_featured: false,
    latest_build: Some(LatestBuild {
        id: 7987,
        version: "2025.511.0",
        display_version: "2025.511.0",
        users: 0,
        created_at: "2025-05-11T15:13:26+00:00",
        update_stream: UpdateStream {
            id: 8,
            name: "web",
            display_name: Some("Web"),
            is_featured: false,
            user_count: None
        }
    }),
    user_count: Some(0)
}
1 Builds: ChanglogBuild {
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
    versions: None
}
2 Builds: ChanglogBuild {
    id: 7986,
    display_version: "2025.508.0",
    users: 0,
    created_at: "2025-05-08T09:20:19+00:00",
    update_stream: UpdateStream {
        id: 8,
        name: "web",
        display_name: Some("Web"),
        is_featured: false,
        user_count: None
    },
    version: Some("2025.508.0"),
    youtube_id: None,
    changelog_entries: Some([ChangelogEntry {
        id: Some(30299),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13186),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13186"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `Tournaments/Official support` with new draft tournament rules"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-01T14:52:16+00:00"),
        github_user: Some(GithubUser {
            display_name: "shdewz",
            github_url: Some("https://github.com/shdewz"),
            github_username: Some("shdewz"),
            id: Some(490),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30301),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13210),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13210"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `Daily Challenge` contributors list"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-01T16:59:26+00:00"),
        github_user: Some(GithubUser {
            display_name: "Hiviexd",
            github_url: Some("https://github.com/Hiviexd"),
            github_username: Some("Hiviexd"),
            id: Some(547),
            osu_username: Some("Hivie"),
            user_id: Some(14102976),
            user_url: Some("https://osu.ppy.sh/users/14102976")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30304),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13213),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13213"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `LGA 2025` announcement news post, wiki article"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-02T14:08:10+00:00"),
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
        id: Some(30305),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13216),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13216"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Fix typo on `The Lazer Grand Arena Returns` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-02T21:23:40+00:00"),
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
        id: Some(30308),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13209),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13209"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update the veto process in related articles"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-04T13:01:08+00:00"),
        github_user: Some(GithubUser {
            display_name: "TheMoMan",
            github_url: Some("https://github.com/TheMoMan"),
            github_username: Some("TheMoMan"),
            id: Some(176),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30309),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13217),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13217"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `osu!taiko World Cup 2025 Concludes` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-04T13:45:10+00:00"),
        github_user: Some(GithubUser {
            display_name: "yasuhosu",
            github_url: Some("https://github.com/yasuhosu"),
            github_username: Some("yasuhosu"),
            id: Some(1000),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30310),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13223),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13223"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `The Lazer Grand Arena Returns` and related articles"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-04T20:22:11+00:00"),
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
        id: Some(30311),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13224),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13224"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update page titles for `LGA 2024/2025`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-04T20:40:42+00:00"),
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
        id: Some(30312),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13222),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13222"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `Featured Artist Track Updates: Krimek` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-04T21:10:25+00:00"),
        github_user: Some(GithubUser {
            display_name: "pishifat",
            github_url: Some("https://github.com/pishifat"),
            github_username: Some("pishifat"),
            id: Some(27),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30313),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13111),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13111"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Merge `oBWC` into `o!bwc`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-05T13:18:55+00:00"),
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
        id: Some(30314),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13227),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13227"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `LGA 2025` links, staff listing"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-06T07:14:27+00:00"),
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
        id: Some(30315),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13208),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13208"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `Expert Global Taiko Showdown 2025 Concludes` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-06T18:46:06+00:00"),
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
        id: Some(30326),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12146),
        github_url: Some("https://github.com/ppy/osu-web/pull/12146"),
        url: None,
        entry_type: "add",
        category: "Team",
        title: Some("Add team chat notification"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-07T13:23:44+00:00"),
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
        id: Some(30327),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12149),
        github_url: Some("https://github.com/ppy/osu-web/pull/12149"),
        url: None,
        entry_type: "fix",
        category: "Code Quality",
        title: Some("Simplify profile hue colour selector css"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-07T13:53:22+00:00"),
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
        id: Some(30328),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13232),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13232"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Fix broken Discord link in `The Followpoint: Spectator` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-07T17:27:05+00:00"),
        github_user: Some(GithubUser {
            display_name: "Joehuu",
            github_url: Some("https://github.com/Joehuu"),
            github_username: Some("Joehuu"),
            id: Some(9),
            osu_username: Some("Joehu"),
            user_id: Some(8549835),
            user_url: Some("https://osu.ppy.sh/users/8549835")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30332),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12151),
        github_url: Some("https://github.com/ppy/osu-web/pull/12151"),
        url: None,
        entry_type: "fix",
        category: "Team",
        title: Some("Keep team leaderboard links sort/ruleset option"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-08T06:03:56+00:00"),
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
        id: Some(30333),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12155),
        github_url: Some("https://github.com/ppy/osu-web/pull/12155"),
        url: None,
        entry_type: "fix",
        category: "Beatmap Listing",
        title: Some("Use bpm data from beatmap when searching"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-05-08T07:40:31+00:00"),
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
    }]),
    versions: None
}
3 Builds: ChanglogBuild {
    id: 7985,
    display_version: "2025.501.0",
    users: 0,
    created_at: "2025-05-01T06:03:09+00:00",
    update_stream: UpdateStream {
        id: 8,
        name: "web",
        display_name: Some("Web"),
        is_featured: false,
        user_count: None
    },
    version: Some("2025.501.0"),
    youtube_id: None,
    changelog_entries: Some([ChangelogEntry {
        id: Some(30229),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12144),
        github_url: Some("https://github.com/ppy/osu-web/pull/12144"),
        url: None,
        entry_type: "add",
        category: "Reliability",
        title: Some("Add alternative url for iptoasn database source"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T08:07:46+00:00"),
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
        id: Some(30230),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12130),
        github_url: Some("https://github.com/ppy/osu-web/pull/12130"),
        url: None,
        entry_type: "fix",
        category: "Beatmap Listing",
        title: Some("Speed up beatmapset es indexing"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T09:45:56+00:00"),
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
        id: Some(30231),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13174),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13174"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `Community`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T09:51:17+00:00"),
        github_user: Some(GithubUser {
            display_name: "diquoks",
            github_url: Some("https://github.com/diquoks"),
            github_username: Some("diquoks"),
            id: Some(1052),
            osu_username: Some("diquoks"),
            user_id: Some(31760756),
            user_url: Some("https://osu.ppy.sh/users/31760756")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30232),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13159),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13159"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Update `osu!stream`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T09:53:21+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30233),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13158),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13158"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Update `Gameplay/Daily challenge`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T09:55:12+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30234),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13078),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13078"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Clean Up `Tournaments/OWC/2020`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T09:57:13+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30235),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13118),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13118"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Add `Community/Reddit`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T09:59:11+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30236),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13088),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13088"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Add `Guides/osu!mania mapping guide`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T10:01:18+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30237),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13083),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13083"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Add `Guides/osu!mania modding guide`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T10:04:27+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30238),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13079),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13079"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Clean Up and Update `Tournaments/OWC/2021`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T10:06:12+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30239),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12145),
        github_url: Some("https://github.com/ppy/osu-web/pull/12145"),
        url: None,
        entry_type: "fix",
        category: "Code Quality",
        title: Some(" Combine channel notification job classes "),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T10:15:31+00:00"),
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
        id: Some(30242),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13151),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13151"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Update `People/osu! wiki maintainers`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T10:53:05+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30243),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12143),
        github_url: Some("https://github.com/ppy/osu-web/pull/12143"),
        url: None,
        entry_type: "fix",
        category: "Misc",
        title: Some("Update mod definitions"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T12:10:27+00:00"),
        github_user: Some(GithubUser {
            display_name: "peppy",
            github_url: Some("https://github.com/peppy"),
            github_username: Some("peppy"),
            id: Some(3),
            osu_username: Some("peppy"),
            user_id: Some(2),
            user_url: Some("https://osu.ppy.sh/users/2")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30245),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13163),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13163"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `Tumblr`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T13:15:54+00:00"),
        github_user: Some(GithubUser {
            display_name: "Nivalyx",
            github_url: Some("https://github.com/Nivalyx"),
            github_username: Some("Nivalyx"),
            id: Some(323),
            osu_username: Some("Niva"),
            user_id: Some(197805),
            user_url: Some("https://osu.ppy.sh/users/197805")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30251),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13178),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13178"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `osu!taiko World Cup 2025: Finals Recap` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-25T12:26:30+00:00"),
        github_user: Some(GithubUser {
            display_name: "yasuhosu",
            github_url: Some("https://github.com/yasuhosu"),
            github_username: Some("yasuhosu"),
            id: Some(1000),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30252),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13183),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13183"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Fix mappool showcase link + small corrections in the `osu!taiko World Cup 2025: Finals Recap` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-25T13:54:10+00:00"),
        github_user: Some(GithubUser {
            display_name: "0x84f",
            github_url: Some("https://github.com/0x84f"),
            github_username: Some("0x84f"),
            id: Some(720),
            osu_username: Some("0x84f"),
            user_id: Some(7944724),
            user_url: Some("https://osu.ppy.sh/users/7944724")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30253),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13184),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13184"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Remove `Support your country with a profile banner` link in `TWC 2025`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-25T18:09:07+00:00"),
        github_user: Some(GithubUser {
            display_name: "iSlodinxOsu",
            github_url: Some("https://github.com/iSlodinxOsu"),
            github_username: Some("iSlodinxOsu"),
            id: Some(836),
            osu_username: Some("Jun Maeda"),
            user_id: Some(8777786),
            user_url: Some("https://osu.ppy.sh/users/8777786")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30259),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13192),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13192"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `TWC 2025` staff listing, results"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-27T14:04:02+00:00"),
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
        id: Some(30262),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13177),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13177"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Update `Community`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T08:43:34+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30263),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13180),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13180"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ES] Add `Tumblr`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T08:48:17+00:00"),
        github_user: Some(GithubUser {
            display_name: "UnknownOsu",
            github_url: Some("https://github.com/UnknownOsu"),
            github_username: Some("UnknownOsu"),
            id: Some(833),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30264),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13182),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13182"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ID] Update `osu! team`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T08:50:23+00:00"),
        github_user: Some(GithubUser {
            display_name: "Nivalyx",
            github_url: Some("https://github.com/Nivalyx"),
            github_username: Some("Nivalyx"),
            id: Some(323),
            osu_username: Some("Niva"),
            user_id: Some(197805),
            user_url: Some("https://osu.ppy.sh/users/197805")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30265),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13185),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13185"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[ID] Fix formatting errors in `Help centre/Client`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T08:52:25+00:00"),
        github_user: Some(GithubUser {
            display_name: "Nivalyx",
            github_url: Some("https://github.com/Nivalyx"),
            github_username: Some("Nivalyx"),
            id: Some(323),
            osu_username: Some("Niva"),
            user_id: Some(197805),
            user_url: Some("https://osu.ppy.sh/users/197805")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30266),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(12619),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/12619"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[DE] Add `Gameplay/Score`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T08:54:38+00:00"),
        github_user: Some(GithubUser {
            display_name: "Malternative3772",
            github_url: Some("https://github.com/Malternative3772"),
            github_username: Some("Malternative3772"),
            id: Some(1107),
            osu_username: Some("Malternative"),
            user_id: Some(21567099),
            user_url: Some("https://osu.ppy.sh/users/21567099")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30267),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13188),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13188"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("[RU] Add `Community`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T09:10:26+00:00"),
        github_user: Some(GithubUser {
            display_name: "diquoks",
            github_url: Some("https://github.com/diquoks"),
            github_username: Some("diquoks"),
            id: Some(1052),
            osu_username: Some("diquoks"),
            user_id: Some(31760756),
            user_url: Some("https://osu.ppy.sh/users/31760756")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30268),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13189),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13189"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Link \"Community\" and \"Projects\" from `Main Page`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T09:14:17+00:00"),
        github_user: Some(GithubUser {
            display_name: "diquoks",
            github_url: Some("https://github.com/diquoks"),
            github_username: Some("diquoks"),
            id: Some(1052),
            osu_username: Some("diquoks"),
            user_id: Some(31760756),
            user_url: Some("https://osu.ppy.sh/users/31760756")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30270),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13154),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13154"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `KEL LAN Tournament 3 Recap` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T18:49:44+00:00"),
        github_user: Some(GithubUser {
            display_name: "0x84f",
            github_url: Some("https://github.com/0x84f"),
            github_username: Some("0x84f"),
            id: Some(720),
            osu_username: Some("0x84f"),
            user_id: Some(7944724),
            user_url: Some("https://osu.ppy.sh/users/7944724")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30271),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13196),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13196"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `New Featured Artist: 1zm8` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T18:56:46+00:00"),
        github_user: Some(GithubUser {
            display_name: "pishifat",
            github_url: Some("https://github.com/pishifat"),
            github_username: Some("pishifat"),
            id: Some(27),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30272),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13197),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13197"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `THMC4` Staff list"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-28T20:00:08+00:00"),
        github_user: Some(GithubUser {
            display_name: "RushFTK",
            github_url: Some("https://github.com/RushFTK"),
            github_username: Some("RushFTK"),
            id: Some(183),
            osu_username: Some("Rush_FTK"),
            user_id: Some(3046856),
            user_url: Some("https://osu.ppy.sh/users/3046856")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30273),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13200),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13200"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Fix typo in `New Featured Artist: 1zm8` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-29T08:20:40+00:00"),
        github_user: Some(GithubUser {
            display_name: "pishifat",
            github_url: Some("https://github.com/pishifat"),
            github_username: Some("pishifat"),
            id: Some(27),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30276),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13204),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13204"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update leaderboard in `Featured Artist playlists` article"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-29T21:07:29+00:00"),
        github_user: Some(GithubUser {
            display_name: "pishifat",
            github_url: Some("https://github.com/pishifat"),
            github_username: Some("pishifat"),
            id: Some(27),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30277),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13175),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13175"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `The Followpoint: Spectator, the osu!catch Mapping Pioneer` news post"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-29T21:50:24+00:00"),
        github_user: Some(GithubUser {
            display_name: "minusQuantumNeko",
            github_url: Some("https://github.com/minusQuantumNeko"),
            github_username: Some("minusQuantumNeko"),
            id: Some(742),
            osu_username: Some("MegaMix"),
            user_id: Some(18152711),
            user_url: Some("https://osu.ppy.sh/users/18152711")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30278),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13171),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13171"),
        url: None,
        entry_type: "add",
        category: "Wiki",
        title: Some("Add `Beatmap tags`"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-30T02:37:39+00:00"),
        github_user: Some(GithubUser {
            display_name: "The-Last-Cookie",
            github_url: Some("https://github.com/The-Last-Cookie"),
            github_username: Some("The-Last-Cookie"),
            id: Some(617),
            osu_username: Some("The_Last_Cookie"),
            user_id: Some(11587828),
            user_url: Some("https://osu.ppy.sh/users/11587828")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30295),
        repository: Some("ppy/osu-wiki"),
        github_pull_request_id: Some(13206),
        github_url: Some("https://github.com/ppy/osu-wiki/pull/13206"),
        url: None,
        entry_type: "fix",
        category: "Wiki",
        title: Some("Update `4WC 2025` staff and team listing"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-30T14:10:35+00:00"),
        github_user: Some(GithubUser {
            display_name: "shdewz",
            github_url: Some("https://github.com/shdewz"),
            github_username: Some("shdewz"),
            id: Some(490),
            osu_username: None,
            user_id: None,
            user_url: None
        }),
        versions: None
    }]),
    versions: None
}
4 Builds: ChanglogBuild {
    id: 7983,
    display_version: "2025.424.0",
    users: 0,
    created_at: "2025-04-24T07:45:30+00:00",
    update_stream: UpdateStream {
        id: 8,
        name: "web",
        display_name: Some("Web"),
        is_featured: false,
        user_count: None
    },
    version: Some("2025.424.0"),
    youtube_id: None,
    changelog_entries: Some([ChangelogEntry {
        id: Some(30202),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12120),
        github_url: Some("https://github.com/ppy/osu-web/pull/12120"),
        url: None,
        entry_type: "add",
        category: "Multiplayer",
        title: Some("Add database table for storing lazer multiplayer room events"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-22T06:26:16+00:00"),
        github_user: Some(GithubUser {
            display_name: "bdach",
            github_url: Some("https://github.com/bdach"),
            github_username: Some("bdach"),
            id: Some(218),
            osu_username: Some("spaceman_atlas"),
            user_id: Some(3035836),
            user_url: Some("https://osu.ppy.sh/users/3035836")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30206),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12087),
        github_url: Some("https://github.com/ppy/osu-web/pull/12087"),
        url: None,
        entry_type: "fix",
        category: "Search",
        title: Some("Include artist track in global search"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-22T08:24:19+00:00"),
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
        id: Some(30208),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12129),
        github_url: Some("https://github.com/ppy/osu-web/pull/12129"),
        url: None,
        entry_type: "fix",
        category: "Localisation",
        title: Some("Fix typo"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-22T09:59:22+00:00"),
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
        id: Some(30209),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12132),
        github_url: Some("https://github.com/ppy/osu-web/pull/12132"),
        url: None,
        entry_type: "fix",
        category: "Misc",
        title: Some("Update elasticsearch-php"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-22T11:00:03+00:00"),
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
    }, ChangelogEntry {
        id: Some(30215),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12124),
        github_url: Some("https://github.com/ppy/osu-web/pull/12124"),
        url: None,
        entry_type: "fix",
        category: "Code Quality",
        title: Some("Remove redundant team id in notification detail"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-23T08:13:43+00:00"),
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
        id: Some(30220),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12125),
        github_url: Some("https://github.com/ppy/osu-web/pull/12125"),
        url: None,
        entry_type: "fix",
        category: "Code Quality",
        title: Some("Simplify linking notification to chat channel"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-23T10:42:20+00:00"),
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
        id: Some(30222),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12136),
        github_url: Some("https://github.com/ppy/osu-web/pull/12136"),
        url: None,
        entry_type: "fix",
        category: "Code Quality",
        title: Some("Move new message notification logic to its own function"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-23T13:21:10+00:00"),
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
        id: Some(30223),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12140),
        github_url: Some("https://github.com/ppy/osu-web/pull/12140"),
        url: None,
        entry_type: "fix",
        category: "Team",
        title: Some("Show detailed team formation date on hover"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T05:22:10+00:00"),
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
        id: Some(30224),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12141),
        github_url: Some("https://github.com/ppy/osu-web/pull/12141"),
        url: None,
        entry_type: "fix",
        category: "Rankings",
        title: Some("Fix ranked score attribute on variant statistics"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T05:44:53+00:00"),
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
        id: Some(30226),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12139),
        github_url: Some("https://github.com/ppy/osu-web/pull/12139"),
        url: None,
        entry_type: "fix",
        category: "Rankings",
        title: Some("Reset variant parameter when switching ruleset"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T06:30:16+00:00"),
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
        id: Some(30227),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12133),
        github_url: Some("https://github.com/ppy/osu-web/pull/12133"),
        url: None,
        entry_type: "fix",
        category: "Team",
        title: Some("Send notification on new team join requests"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T07:21:41+00:00"),
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
        id: Some(30228),
        repository: Some("ppy/osu-web"),
        github_pull_request_id: Some(12142),
        github_url: Some("https://github.com/ppy/osu-web/pull/12142"),
        url: None,
        entry_type: "fix",
        category: "Reliability",
        title: Some("Fix type of beatmapset id column on downloads table"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T07:30:49+00:00"),
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
    }]),
    versions: None
}
5 Builds: ChanglogBuild {
    id: 7982,
    display_version: "2025.424.0",
    users: 1483,
    created_at: "2025-04-24T07:06:36+00:00",
    update_stream: UpdateStream {
        id: 7,
        name: "lazer",
        display_name: Some("Lazer"),
        is_featured: false,
        user_count: None
    },
    version: Some("2025.424.0"),
    youtube_id: None,
    changelog_entries: Some([ChangelogEntry {
        id: Some(30191),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32893),
        github_url: Some("https://github.com/ppy/osu/pull/32893"),
        url: None,
        entry_type: "fix",
        category: "Gameplay",
        title: Some("Allow toggling leaderboard visibility in replays"),
        message: Some(""),
        message_html: None,
        major: true,
        created_at: Some("2025-04-21T07:27:37+00:00"),
        github_user: Some(GithubUser {
            display_name: "bdach",
            github_url: Some("https://github.com/bdach"),
            github_username: Some("bdach"),
            id: Some(218),
            osu_username: Some("spaceman_atlas"),
            user_id: Some(3035836),
            user_url: Some("https://osu.ppy.sh/users/3035836")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30193),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32884),
        github_url: Some("https://github.com/ppy/osu/pull/32884"),
        url: None,
        entry_type: "fix",
        category: "Settings",
        title: Some("Fix slider tooltip text not updating with current value"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-21T09:21:29+00:00"),
        github_user: Some(GithubUser {
            display_name: "SchiavoAnto",
            github_url: Some("https://github.com/SchiavoAnto"),
            github_username: Some("SchiavoAnto"),
            id: Some(1015),
            osu_username: Some("TheHardCoder"),
            user_id: Some(25929686),
            user_url: Some("https://osu.ppy.sh/users/25929686")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30201),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32894),
        github_url: Some("https://github.com/ppy/osu/pull/32894"),
        url: None,
        entry_type: "fix",
        category: "Editor",
        title: Some("Fix slider repeat arrows appearing too early in editor when hit markers are enabled"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-21T18:21:40+00:00"),
        github_user: Some(GithubUser {
            display_name: "bdach",
            github_url: Some("https://github.com/bdach"),
            github_username: Some("bdach"),
            id: Some(218),
            osu_username: Some("spaceman_atlas"),
            user_id: Some(3035836),
            user_url: Some("https://osu.ppy.sh/users/3035836")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30212),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32912),
        github_url: Some("https://github.com/ppy/osu/pull/32912"),
        url: None,
        entry_type: "fix",
        category: "Multiplayer",
        title: Some("Fix free mod selection sub-button being clickable even if the main button isn't"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-23T06:37:55+00:00"),
        github_user: Some(GithubUser {
            display_name: "bdach",
            github_url: Some("https://github.com/bdach"),
            github_username: Some("bdach"),
            id: Some(218),
            osu_username: Some("spaceman_atlas"),
            user_id: Some(3035836),
            user_url: Some("https://osu.ppy.sh/users/3035836")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30214),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32923),
        github_url: Some("https://github.com/ppy/osu/pull/32923"),
        url: None,
        entry_type: "fix",
        category: "Multiplayer",
        title: Some("Fix multiplayer settings screen closing unexpectedly on user ready state change"),
        message: Some(""),
        message_html: None,
        major: true,
        created_at: Some("2025-04-23T07:47:56+00:00"),
        github_user: Some(GithubUser {
            display_name: "smoogipoo",
            github_url: Some("https://github.com/smoogipoo"),
            github_username: Some("smoogipoo"),
            id: Some(5),
            osu_username: Some("smoogipoo"),
            user_id: Some(1040328),
            user_url: Some("https://osu.ppy.sh/users/1040328")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30218),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32909),
        github_url: Some("https://github.com/ppy/osu/pull/32909"),
        url: None,
        entry_type: "fix",
        category: "UI",
        title: Some("Fix daily challenge marker text spacing"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-23T09:24:56+00:00"),
        github_user: Some(GithubUser {
            display_name: "bdach",
            github_url: Some("https://github.com/bdach"),
            github_username: Some("bdach"),
            id: Some(218),
            osu_username: Some("spaceman_atlas"),
            user_id: Some(3035836),
            user_url: Some("https://osu.ppy.sh/users/3035836")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30219),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32924),
        github_url: Some("https://github.com/ppy/osu/pull/32924"),
        url: None,
        entry_type: "fix",
        category: "Multiplayer",
        title: Some("Show true beatmap background when viewing historical multiplayer results"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-23T10:42:00+00:00"),
        github_user: Some(GithubUser {
            display_name: "smoogipoo",
            github_url: Some("https://github.com/smoogipoo"),
            github_username: Some("smoogipoo"),
            id: Some(5),
            osu_username: Some("smoogipoo"),
            user_id: Some(1040328),
            user_url: Some("https://osu.ppy.sh/users/1040328")
        }),
        versions: None
    }, ChangelogEntry {
        id: Some(30225),
        repository: Some("ppy/osu"),
        github_pull_request_id: Some(32922),
        github_url: Some("https://github.com/ppy/osu/pull/32922"),
        url: None,
        entry_type: "fix",
        category: "Settings",
        title: Some("Fix tablet settings adjusting with too much precision"),
        message: Some(""),
        message_html: None,
        major: false,
        created_at: Some("2025-04-24T06:23:56+00:00"),
        github_user: Some(GithubUser {
            display_name: "peppy",
            github_url: Some("https://github.com/peppy"),
            github_username: Some("peppy"),
            id: Some(3),
            osu_username: Some("peppy"),
            user_id: Some(2),
            user_url: Some("https://osu.ppy.sh/users/2")
        }),
        versions: None
    }]),
    versions: None
}
Search: Search {
    stream: None,
    from: None,
    to: None,
    max_id: None,
    limit: 21
}
*/
