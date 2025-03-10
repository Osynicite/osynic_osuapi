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
    let scores = client
        .beatmaps
        .get_scores(4801662, None, Some(Mode::Catch), None, None)
        .await?;
    println!("{:?}", scores);

    Ok(())
}

/*
ReqwestBeatmaps get_Scores
BeatmapScores {
    scores: [Score {
        accuracy: 1.0,
        best_id: Some(218650806),
        created_at: "2024-10-23T17:24:57Z",
        id: 218650806,
        max_combo: 911,
        mode: Catch,
        mode_int: 2,
        mods: ["HD"],
        passed: true,
        perfect: true,
        pp: 96.3353,
        rank: XH,
        replay: true,
        score: 20727177,
        statistics: Statistics {
            count_100: 32,
            count_300: 879,
            count_50: 597,
            count_geki: None,
            count_katu: 0,
            count_miss: 0
        },
        ranking_type: "score_best_fruits",
        user_id: 869471,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/869471?1654883742.jpeg",
            country_code: "ES",
            default_group: "default",
            id: 869471,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-03-07T00:45:26+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "DPJ9fuegos",
            country: Country {
                code: "ES",
                name: "Spain"
            },
            cover: Cover {
                custom_url: None,
                url: "https://assets.ppy.sh/user-cover-presets/8/eda177e3c4eb0889e24f58c454af47708b0acd612bff984f6ac295e8427681c5.jpeg",
                id: Some("8")
            },
            team: None
        })
    }, Score {
        accuracy: 1.0,
        best_id: Some(218655611),
        created_at: "2024-10-24T05:42:04Z",
        id: 218655611,
        max_combo: 911,
        mode: Catch,
        mode_int: 2,
        mods: ["HD"],
        passed: true,
        perfect: true,
        pp: 96.3353,
        rank: XH,
        replay: true,
        score: 20727177,
        statistics: Statistics {
            count_100: 32,
            count_300: 879,
            count_50: 597,
            count_geki: None,
            count_katu: 0,
            count_miss: 0
        },
        ranking_type: "score_best_fruits",
        user_id: 1375955,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/1375955?1728883293.png",
            country_code: "US",
            default_group: "default",
            id: 1375955,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: true,
            is_supporter: true,
            last_visit: "2025-03-07T05:39:55+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "Zak",
            country: Country {
                code: "US",
                name: "United States"
            },
            cover: Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/1375955/e7c320b4d6e4fcf3cf131934566eec0fa6936b23cf4332ae5b71a8b05a024af5.jpeg"),
                url: "https://assets.ppy.sh/user-profile-covers/1375955/e7c320b4d6e4fcf3cf131934566eec0fa6936b23cf4332ae5b71a8b05a024af5.jpeg",
                id: None
            },
            team: None
        })
    }, Score {
        accuracy: 1.0,
        best_id: Some(218652437),
        created_at: "2024-10-23T20:40:27Z",
        id: 218652437,
        max_combo: 911,
        mode: Catch,
        mode_int: 2,
        mods: ["PF"],
        passed: true,
        perfect: true,
        pp: 80.2794,
        rank: X,
        replay: true,
        score: 19569782,
        statistics: Statistics {
            count_100: 32,
            count_300: 879,
            count_50: 597,
            count_geki: None,
            count_katu: 0,
            count_miss: 0
        },
        ranking_type: "score_best_fruits",
        user_id: 14477534,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/14477534?1741043325.jpeg",
            country_code: "RS",
            default_group: "default",
            id: 14477534,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: true,
            last_visit: "2025-03-07T00:51:29+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "optm",
            country: Country {
                code: "RS",
                name: "Serbia"
            },
            cover: Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/14477534/ce2cb985b4db886dfdf2352859d5ac5c2b12ad272ee32f32c2ed5fb001daf24d.gif"),
                url: "https://assets.ppy.sh/user-profile-covers/14477534/ce2cb985b4db886dfdf2352859d5ac5c2b12ad272ee32f32c2ed5fb001daf24d.gif",
                id: None
            },
            team: Some(Team {
                flag_url: "https://assets.ppy.sh/teams/flag/649/108cd4439bc2a5206a0c3e529e147d2897bb41afbd904b46ac358410b20a0b73.gif",
                id: 649,
                name: "squadra scimmie",
                short_name: "79"
            })
        })
    }, Score {
        accuracy: 1.0,
        best_id: Some(218984888),
        created_at: "2024-11-22T08:20:40Z",
        id: 218984888,
        max_combo: 911,
        mode: Catch,
        mode_int: 2,
        mods: [],
        passed: true,
        perfect: true,
        pp: 80.2794,
        rank: X,
        replay: true,
        score: 19569782,
        statistics: Statistics {
            count_100: 32,
            count_300: 879,
            count_50: 597,
            count_geki: None,
            count_katu: 0,
            count_miss: 0
        },
        ranking_type: "score_best_fruits",
        user_id: 4577865,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4577865?1738113771.gif",
            country_code: "PH",
            default_group: "default",
            id: 4577865,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-03-07T04:57:47+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "Chloebe",
            country: Country {
                code: "PH",
                name: "Philippines"
            },
            cover: Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4577865/de279582bf4e86ef588ff48da44a6dc3d16111a3b97e3953d28b73c18b18940f.jpeg"),
                url: "https://assets.ppy.sh/user-profile-covers/4577865/de279582bf4e86ef588ff48da44a6dc3d16111a3b97e3953d28b73c18b18940f.jpeg",
                id: None
            },
            team: Some(Team {
                flag_url: "https://assets.ppy.sh/teams/flag/7/b12f51d25f89767fc05bbd503a26e7211a460fa2db01282c1c2999b611de4928.png",
                id: 7,
                name: "osu! Philippines",
                short_name: "OPH"
            })
        })
    }, Score {
        accuracy: 1.0,
        best_id: Some(219844868),
        created_at: "2025-02-13T15:53:44Z",
        id: 219844868,
        max_combo: 911,
        mode: Catch,
        mode_int: 2,
        mods: ["PF"],
        passed: true,
        perfect: true,
        pp: 80.2794,
        rank: X,
        replay: true,
        score: 19569782,
        statistics: Statistics {
            count_100: 32,
            count_300: 879,
            count_50: 597,
            count_geki: None,
            count_katu: 0,
            count_miss: 0
        },
        ranking_type: "score_best_fruits",
        user_id: 2400918,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/2400918?1730679337.jpeg",
            country_code: "JP",
            default_group: "default",
            id: 2400918,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: true,
            last_visit: "2025-03-06T22:20:03+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "syu",
            country: Country {
                code: "JP",
                name: "Japan"
            },
            cover: Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/2400918/11520d29cfeab2da1012a443b32879843c262aabb5145e1753a6cd4d2ff649ee.jpeg"),
                url: "https://assets.ppy.sh/user-profile-covers/2400918/11520d29cfeab2da1012a443b32879843c262aabb5145e1753a6cd4d2ff649ee.jpeg",
                id: None
            },
            team: Some(Team {
                flag_url: "https://assets.ppy.sh/teams/flag/5956/50b612dabacc7f98704579782ef9f9dc35a092da0f34026f64fcd8a4f3be074a.png",
                id: 5956,
                name: "osu!catch Convert Enjoyer",
                short_name: "CONV"
            })
        })
    }, Score {
        accuracy: 0.996684350132626,
        best_id: Some(218839926),
        created_at: "2024-11-09T19:43:34Z",
        id: 218839926,
        max_combo: 480,
        mode: Catch,
        mode_int: 2,
        mods: ["HD"],
        passed: true,
        perfect: false,
        pp: 51.7066,
        rank: SH,
        replay: true,
        score: 8807008,
        statistics: Statistics {
            count_100: 32,
            count_300: 876,
            count_50: 595,
            count_geki: None,
            count_katu: 2,
            count_miss: 3
        },
        ranking_type: "score_best_fruits",
        user_id: 13382488,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/13382488?1741110998.jpeg",
            country_code: "US",
            default_group: "default",
            id: 13382488,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-03-07T04:45:23+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "Rickster_rick",
            country: Country {
                code: "US",
                name: "United States"
            },
            cover: Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/13382488/4c726721bb2c7621c00b43dbaa7d6e4d88342ed589d306b2a60864b9e23964db.jpeg"),
                url: "https://assets.ppy.sh/user-profile-covers/13382488/4c726721bb2c7621c00b43dbaa7d6e4d88342ed589d306b2a60864b9e23964db.jpeg",
                id: None
            },
            team: None
        })
    }, Score {
        accuracy: 0.9960212201591512,
        best_id: Some(218904272),
        created_at: "2024-11-15T05:59:43Z",
        id: 218904272,
        max_combo: 480,
        mode: Catch,
        mode_int: 2,
        mods: [],
        passed: true,
        perfect: false,
        pp: 41.6434,
        rank: S,
        replay: true,
        score: 7422306,
        statistics: Statistics {
            count_100: 32,
            count_300: 875,
            count_50: 595,
            count_geki: None,
            count_katu: 2,
            count_miss: 4
        },
        ranking_type: "score_best_fruits",
        user_id: 10932328,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/10932328?1707817072.jpeg",
            country_code: "CN",
            default_group: "default",
            id: 10932328,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: true,
            last_visit: "2025-03-07T04:17:39+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "kcwt",
            country: Country {
                code: "CN",
                name: "China"
            },
            cover: Cover {
                custom_url: None,
                url: "https://assets.ppy.sh/user-cover-presets/1/df28696b58541a9e67f6755918951d542d93bdf1da41720fcca2fd2c1ea8cf51.jpeg",
                id: Some("1")
            },
            team: Some(Team {
                flag_url: "https://assets.ppy.sh/teams/flag/3761/c8435c6881219d5d71c003535873e96b6a034ccb077b65d80e64c611aa173920.png",
                id: 3761,
                name: "barbecue stand",
                short_name: "BBQ"
            })
        })
    }, Score {
        accuracy: 0.9840848806366048,
        best_id: None,
        created_at: "2024-10-23T21:05:29Z",
        id: 3731928946,
        max_combo: 239,
        mode: Catch,
        mode_int: 2,
        mods: [],
        passed: true,
        perfect: false,
        pp: 15.5346,
        rank: S,
        replay: true,
        score: 0,
        statistics: Statistics {
            count_100: 32,
            count_300: 863,
            count_50: 589,
            count_geki: None,
            count_katu: 8,
            count_miss: 16
        },
        ranking_type: "solo_score",
        user_id: 36528993,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/36528993?1730976075.jpeg",
            country_code: "FI",
            default_group: "default",
            id: 36528993,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-03-01T16:36:50+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "Juibbi",
            country: Country {
                code: "FI",
                name: "Finland"
            },
            cover: Cover {
                custom_url: None,
                url: "https://assets.ppy.sh/user-cover-presets/3/32ddb3eb261e38a82067f9ef4ea96c12f6abf8bd228e6413330f9d351420301b.jpeg",
                id: Some("3")
            },
            team: None
        })
    }, Score {
        accuracy: 0.9847480106100795,
        best_id: Some(218672285),
        created_at: "2024-10-25T19:42:01Z",
        id: 218672285,
        max_combo: 270,
        mode: Catch,
        mode_int: 2,
        mods: [],
        passed: true,
        perfect: false,
        pp: 16.1743,
        rank: S,
        replay: true,
        score: 3305580,
        statistics: Statistics {
            count_100: 32,
            count_300: 861,
            count_50: 592,
            count_geki: None,
            count_katu: 5,
            count_miss: 18
        },
        ranking_type: "score_best_fruits",
        user_id: 4296063,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4296063?1452191303.png",
            country_code: "GB",
            default_group: "default",
            id: 4296063,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-02-20T22:36:45+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "Ch3zGr8r",
            country: Country {
                code: "GB",
                name: "United Kingdom"
            },
            cover: Cover {
                custom_url: None,
                url: "https://assets.ppy.sh/user-cover-presets/8/eda177e3c4eb0889e24f58c454af47708b0acd612bff984f6ac295e8427681c5.jpeg",
                id: Some("8")
            },
            team: None
        })
    }, Score {
        accuracy: 0.980106100795756,
        best_id: Some(220037892),
        created_at: "2025-03-03T05:10:56Z",
        id: 220037892,
        max_combo: 238,
        mode: Catch,
        mode_int: 2,
        mods: [],
        passed: true,
        perfect: false,
        pp: 14.1953,
        rank: S,
        replay: true,
        score: 3218918,
        statistics: Statistics {
            count_100: 32,
            count_300: 861,
            count_50: 585,
            count_geki: None,
            count_katu: 12,
            count_miss: 18
        },
        ranking_type: "score_best_fruits",
        user_id: 31175842,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/31175842?1738717996.jpeg",
            country_code: "CN",
            default_group: "default",
            id: 31175842,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-03-07T05:29:56+00:00",
            pm_friends_only: false,
            profile_colour: None,
            username: "Islatri",
            country: Country {
                code: "CN",
                name: "China"
            },
            cover: Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png"),
                url: "https://assets.ppy.sh/user-profile-covers/31175842/e7918a412b7386b09503a5a2050825c717cfacdeceb4bed59bd8a4ba064946ab.png",
                id: None
            },
            team: None
        })
    }],
    position: None
}

*/
