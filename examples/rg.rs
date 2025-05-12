// Get Ranking
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::ranking::IRanking;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::model::ranking::enums::ranking_type::RankingType;

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
    let rankings = client
        .ranking
        .get_ranking(
            Mode::Catch,
            RankingType::Score,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    println!("{:?}", rankings);
    Ok(())
}

/*
ReqwestRankings get_ranking
Rankings {
    beatmapsets: None,
    cursor: Cursor {
        page: 2
    },
    ranking: [Statistics {
        count_100: 6278161,
        count_300: 144910241,
        count_50: 87919206,
        count_miss: 999784,
        country_rank: None,
        level: Level {
            current: 122,
            progress: 19
        },
        global_rank: Some(540),
        global_rank_exp: None,
        pp: 10977.6,
        pp_exp: Some(0.0),
        ranked_score: 1347234502083,
        hit_accuracy: 99.8432,
        play_count: 397484,
        play_time: 28837418,
        total_score: 2246459172167,
        total_hits: 239107608,
        maximum_combo: 8107,
        replays_watched_by_others: 10104,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 73479,
            ssh: 30797,
            s: 9304,
            sh: 2683,
            a: 236
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/2400918?1730679337.jpeg",
            country: Some(Country {
                code: "JP",
                name: "Japan"
            }),
            country_code: "JP",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/2400918/11520d29cfeab2da1012a443b32879843c262aabb5145e1753a6cd4d2ff649ee.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/2400918/11520d29cfeab2da1012a443b32879843c262aabb5145e1753a6cd4d2ff649ee.jpeg"
            }),
            default_group: "default",
            id: 2400918,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-12T03:29:28+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "syu"
        })
    }, Statistics {
        count_100: 7129134,
        count_300: 181800526,
        count_50: 88182978,
        count_miss: 1541578,
        country_rank: None,
        level: Level {
            current: 126,
            progress: 26
        },
        global_rank: Some(175),
        global_rank_exp: None,
        pp: 15487.7,
        pp_exp: Some(0.0),
        ranked_score: 1052513291483,
        hit_accuracy: 99.9143,
        play_count: 577606,
        play_time: 32015551,
        total_score: 2653649391801,
        total_hits: 277112638,
        maximum_combo: 7823,
        replays_watched_by_others: 86049,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 27847,
            ssh: 27513,
            s: 6768,
            sh: 14118,
            a: 166
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/3097304?1730539933.jpeg",
            country: Some(Country {
                code: "KR",
                name: "South Korea"
            }),
            country_code: "KR",
            cover: Some(Cover {
                custom_url: None,
                id: Some("3"),
                url: "https://assets.ppy.sh/user-cover-presets/3/32ddb3eb261e38a82067f9ef4ea96c12f6abf8bd228e6413330f9d351420301b.jpeg"
            }),
            default_group: "default",
            id: 3097304,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Abstract-"
        })
    }, Statistics {
        count_100: 7494058,
        count_300: 123557757,
        count_50: 97946133,
        count_miss: 1051739,
        country_rank: None,
        level: Level {
            current: 117,
            progress: 3
        },
        global_rank: Some(655),
        global_rank_exp: None,
        pp: 10125.4,
        pp_exp: Some(0.0),
        ranked_score: 1007282579535,
        hit_accuracy: 99.9604,
        play_count: 411785,
        play_time: 29325440,
        total_score: 1730655212538,
        total_hits: 228997948,
        maximum_combo: 6886,
        replays_watched_by_others: 111241,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1000,
            ssh: 102046,
            s: 106,
            sh: 28,
            a: 0
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4568537?1743523736.gif",
            country: Some(Country {
                code: "BR",
                name: "Brazil"
            }),
            country_code: "BR",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4568537/47f85ef50f6432d3d48bde838af5c58f785252a2d4640e0457070077487800c6.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/4568537/47f85ef50f6432d3d48bde838af5c58f785252a2d4640e0457070077487800c6.png"
            }),
            default_group: "default",
            id: 4568537,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Predominador"
        })
    }, Statistics {
        count_100: 6900664,
        count_300: 146360721,
        count_50: 74485764,
        count_miss: 1812561,
        country_rank: None,
        level: Level {
            current: 117,
            progress: 76
        },
        global_rank: Some(427),
        global_rank_exp: None,
        pp: 11966.9,
        pp_exp: Some(0.0),
        ranked_score: 968066169026,
        hit_accuracy: 99.8418,
        play_count: 468020,
        play_time: 27529488,
        total_score: 1802882299032,
        total_hits: 227747149,
        maximum_combo: 9225,
        replays_watched_by_others: 26184,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 830,
            ssh: 61832,
            s: 235,
            sh: 16990,
            a: 967
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/1375955?1728883293.png",
            country: Some(Country {
                code: "US",
                name: "United States"
            }),
            country_code: "US",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/1375955/e7c320b4d6e4fcf3cf131934566eec0fa6936b23cf4332ae5b71a8b05a024af5.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/1375955/e7c320b4d6e4fcf3cf131934566eec0fa6936b23cf4332ae5b71a8b05a024af5.jpeg"
            }),
            default_group: "default",
            id: 1375955,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T04:13:19+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Zak"
        })
    }, Statistics {
        count_100: 2934208,
        count_300: 55571553,
        count_50: 43424511,
        count_miss: 256585,
        country_rank: None,
        level: Level {
            current: 107,
            progress: 46
        },
        global_rank: Some(1337),
        global_rank_exp: None,
        pp: 7083.31,
        pp_exp: Some(0.0),
        ranked_score: 626990765965,
        hit_accuracy: 99.5793,
        play_count: 147730,
        play_time: 12339717,
        total_score: 773912930578,
        total_hits: 101930272,
        maximum_combo: 9225,
        replays_watched_by_others: 2322,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 73145,
            ssh: 2651,
            s: 9626,
            sh: 232,
            a: 251
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/13279756?1746952274.jpeg",
            country: Some(Country {
                code: "ES",
                name: "Spain"
            }),
            country_code: "ES",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/13279756/647d1cc2f0a9ce4873d42291d6dd95daf46fc33bd7cfc897c5e8e6845dbc040f.jpeg"),
                id: Some("10"),
                url: "https://assets.ppy.sh/user-cover-presets/10/1e6e3ae468d87bfec64acee31bce234c52c35fbb65cabcd6dabbbc15d4f3f8ee.png"
            }),
            default_group: "default",
            id: 13279756,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: None,
            pm_friends_only: true,
            profile_colour: None,
            username: "Systine"
        })
    }, Statistics {
        count_100: 4399383,
        count_300: 76760330,
        count_50: 61050267,
        count_miss: 659349,
        country_rank: None,
        level: Level {
            current: 109,
            progress: 11
        },
        global_rank: Some(652),
        global_rank_exp: None,
        pp: 10143.8,
        pp_exp: Some(0.0),
        ranked_score: 575532908229,
        hit_accuracy: 99.6404,
        play_count: 271072,
        play_time: 17333389,
        total_score: 938522749821,
        total_hits: 142209980,
        maximum_combo: 5908,
        replays_watched_by_others: 29174,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 39618,
            ssh: 43385,
            s: 584,
            sh: 1504,
            a: 38
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/5163523?1743357459.png",
            country: Some(Country {
                code: "HK",
                name: "Hong Kong"
            }),
            country_code: "HK",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/5163523/fe5a59980779b7fd9fef91dd30e10626e58876a371f16a3adc96ea7fa5efcb2b.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/5163523/fe5a59980779b7fd9fef91dd30e10626e58876a371f16a3adc96ea7fa5efcb2b.png"
            }),
            default_group: "default",
            id: 5163523,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: true,
            profile_colour: None,
            username: "Novoids"
        })
    }, Statistics {
        count_100: 2543854,
        count_300: 46703407,
        count_50: 27674602,
        count_miss: 353896,
        country_rank: None,
        level: Level {
            current: 108,
            progress: 36
        },
        global_rank: Some(1466),
        global_rank_exp: None,
        pp: 6728.55,
        pp_exp: Some(0.0),
        ranked_score: 563549236661,
        hit_accuracy: 99.9941,
        play_count: 144400,
        play_time: 9858322,
        total_score: 863342543694,
        total_hits: 76921863,
        maximum_combo: 9061,
        replays_watched_by_others: 6785,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 189,
            ssh: 21992,
            s: 106,
            sh: 327,
            a: 6
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/869471?1654883742.jpeg",
            country: Some(Country {
                code: "ES",
                name: "Spain"
            }),
            country_code: "ES",
            cover: Some(Cover {
                custom_url: None,
                id: Some("8"),
                url: "https://assets.ppy.sh/user-cover-presets/8/eda177e3c4eb0889e24f58c454af47708b0acd612bff984f6ac295e8427681c5.jpeg"
            }),
            default_group: "default",
            id: 869471,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-12T00:47:58+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "DPJ9fuegos"
        })
    }, Statistics {
        count_100: 2423251,
        count_300: 55688997,
        count_50: 38642119,
        count_miss: 461956,
        country_rank: None,
        level: Level {
            current: 107,
            progress: 36
        },
        global_rank: Some(1892),
        global_rank_exp: None,
        pp: 5673.5,
        pp_exp: Some(0.0),
        ranked_score: 511212640457,
        hit_accuracy: 99.0968,
        play_count: 183813,
        play_time: 12253590,
        total_score: 763381481338,
        total_hits: 96754367,
        maximum_combo: 3974,
        replays_watched_by_others: 4841,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 46659,
            ssh: 4098,
            s: 8680,
            sh: 260,
            a: 181
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/10826368?1745339503.gif",
            country: Some(Country {
                code: "MY",
                name: "Malaysia"
            }),
            country_code: "MY",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/10826368/48a71871e0c6931c16f0afff377bf73360058fe002df68ac88a098dc68b99820.gif"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/10826368/48a71871e0c6931c16f0afff377bf73360058fe002df68ac88a098dc68b99820.gif"
            }),
            default_group: "default",
            id: 10826368,
            is_active: true,
            is_bot: false,
            is_online: true,
            is_supporter: true,
            last_visit: Some("2025-05-12T07:55:37+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Green Ghost"
        })
    }, Statistics {
        count_100: 2826979,
        count_300: 43420756,
        count_50: 23091036,
        count_miss: 738637,
        country_rank: None,
        level: Level {
            current: 109,
            progress: 17
        },
        global_rank: Some(223),
        global_rank_exp: None,
        pp: 14659.9,
        pp_exp: Some(0.0),
        ranked_score: 509336262283,
        hit_accuracy: 99.9537,
        play_count: 130075,
        play_time: 9091804,
        total_score: 944511583227,
        total_hits: 69338771,
        maximum_combo: 9281,
        replays_watched_by_others: 29708,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 9221,
            ssh: 12620,
            s: 2800,
            sh: 2829,
            a: 323
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/5302804?1746696486.gif",
            country: Some(Country {
                code: "FI",
                name: "Finland"
            }),
            country_code: "FI",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/5302804/a612d431ebb7b5b6b37daab414a19fe1eaad5b042237268f45af27e14df72c62.gif"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/5302804/a612d431ebb7b5b6b37daab414a19fe1eaad5b042237268f45af27e14df72c62.gif"
            }),
            default_group: "default",
            id: 5302804,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T23:46:16+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Nikolai"
        })
    }, Statistics {
        count_100: 3320762,
        count_300: 69514852,
        count_50: 35687721,
        count_miss: 510231,
        country_rank: None,
        level: Level {
            current: 110,
            progress: 28
        },
        global_rank: Some(663),
        global_rank_exp: None,
        pp: 10081.4,
        pp_exp: Some(0.0),
        ranked_score: 472157847113,
        hit_accuracy: 99.8323,
        play_count: 184561,
        play_time: 13443665,
        total_score: 1055143550372,
        total_hits: 108523335,
        maximum_combo: 9225,
        replays_watched_by_others: 15929,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 22232,
            ssh: 8441,
            s: 3639,
            sh: 627,
            a: 154
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/1101600?1740255681.png",
            country: Some(Country {
                code: "DE",
                name: "Germany"
            }),
            country_code: "DE",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/1101600/7daa27fec64b15e34c608354ac0e10b294414d79043e50c770a88f13820fe3d1.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/1101600/7daa27fec64b15e34c608354ac0e10b294414d79043e50c770a88f13820fe3d1.jpeg"
            }),
            default_group: "loved",
            id: 1101600,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T00:29:54+00:00"),
            pm_friends_only: false,
            profile_colour: Some("#FFD1DC"),
            username: "Tenshichan"
        })
    }, Statistics {
        count_100: 4827982,
        count_300: 86772026,
        count_50: 48721038,
        count_miss: 1999863,
        country_rank: None,
        level: Level {
            current: 111,
            progress: 54
        },
        global_rank: Some(619),
        global_rank_exp: None,
        pp: 10376.4,
        pp_exp: Some(0.0),
        ranked_score: 430828060817,
        hit_accuracy: 100.0,
        play_count: 261239,
        play_time: 17933064,
        total_score: 1181253775863,
        total_hits: 140321046,
        maximum_combo: 9225,
        replays_watched_by_others: 10616,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 4783,
            ssh: 27852,
            s: 4348,
            sh: 659,
            a: 6
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/1952803?1732746306.jpeg",
            country: Some(Country {
                code: "TW",
                name: "Taiwan"
            }),
            country_code: "TW",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/1952803/3974e9a1d843cf97e0109b646ea759c150ea4824b7579f17d0a2482f56460ec0.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/1952803/3974e9a1d843cf97e0109b646ea759c150ea4824b7579f17d0a2482f56460ec0.jpeg"
            }),
            default_group: "default",
            id: 1952803,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T05:51:02+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Oktavia"
        })
    }, Statistics {
        count_100: 1718558,
        count_300: 50871626,
        count_50: 22722803,
        count_miss: 493677,
        country_rank: None,
        level: Level {
            current: 106,
            progress: 73
        },
        global_rank: Some(1089),
        global_rank_exp: None,
        pp: 7945.96,
        pp_exp: Some(0.0),
        ranked_score: 422431182247,
        hit_accuracy: 99.6328,
        play_count: 113852,
        play_time: 8944112,
        total_score: 700708685881,
        total_hits: 75312987,
        maximum_combo: 5908,
        replays_watched_by_others: 2990,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 709,
            ssh: 6839,
            s: 1263,
            sh: 17551,
            a: 382
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/538717?1726388082.png",
            country: Some(Country {
                code: "JP",
                name: "Japan"
            }),
            country_code: "JP",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/538717/144ee81aa35d7dec94505c4a7b391de83be9a28cd04f7e2e7569b92d53ca933a.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/538717/144ee81aa35d7dec94505c4a7b391de83be9a28cd04f7e2e7569b92d53ca933a.jpeg"
            }),
            default_group: "default",
            id: 538717,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T22:50:31+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "thedted"
        })
    }, Statistics {
        count_100: 3612618,
        count_300: 59450966,
        count_50: 27906780,
        count_miss: 9994952,
        country_rank: None,
        level: Level {
            current: 106,
            progress: 43
        },
        global_rank: Some(520),
        global_rank_exp: None,
        pp: 11122.8,
        pp_exp: Some(0.0),
        ranked_score: 380968502212,
        hit_accuracy: 99.6925,
        play_count: 148482,
        play_time: 10405652,
        total_score: 670944621293,
        total_hits: 90970364,
        maximum_combo: 7846,
        replays_watched_by_others: 4790,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1490,
            ssh: 18577,
            s: 3769,
            sh: 18314,
            a: 1380
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/17819541?1718287717.jpeg",
            country: Some(Country {
                code: "FR",
                name: "France"
            }),
            country_code: "FR",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/17819541/3fa4a0cb23268f23c642e97cfbce3b948dae96d5ed1525ca3b928e547750270e.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/17819541/3fa4a0cb23268f23c642e97cfbce3b948dae96d5ed1525ca3b928e547750270e.png"
            }),
            default_group: "default",
            id: 17819541,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-11T23:48:10+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "ERROR CODE 0x7A"
        })
    }, Statistics {
        count_100: 3356175,
        count_300: 60094083,
        count_50: 25351031,
        count_miss: 1243349,
        country_rank: None,
        level: Level {
            current: 109,
            progress: 97
        },
        global_rank: Some(78),
        global_rank_exp: None,
        pp: 18159.7,
        pp_exp: Some(0.0),
        ranked_score: 373963500535,
        hit_accuracy: 99.9785,
        play_count: 174532,
        play_time: 11226325,
        total_score: 1024575456471,
        total_hits: 88801289,
        maximum_combo: 9225,
        replays_watched_by_others: 14140,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 7403,
            ssh: 13536,
            s: 1972,
            sh: 509,
            a: 95
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/12740132?1746920729.jpeg",
            country: Some(Country {
                code: "FI",
                name: "Finland"
            }),
            country_code: "FI",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/12740132/8c5a4e9cab465f4294d187e41bdfd52bab77799a64df0e7c5f283835ea348da4.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/12740132/8c5a4e9cab465f4294d187e41bdfd52bab77799a64df0e7c5f283835ea348da4.png"
            }),
            default_group: "default",
            id: 12740132,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-10T23:44:25+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "JonZku"
        })
    }, Statistics {
        count_100: 3695314,
        count_300: 53782096,
        count_50: 16775953,
        count_miss: 1927649,
        country_rank: None,
        level: Level {
            current: 111,
            progress: 34
        },
        global_rank: Some(5),
        global_rank_exp: None,
        pp: 24424.4,
        pp_exp: Some(0.0),
        ranked_score: 372237515780,
        hit_accuracy: 99.9706,
        play_count: 133947,
        play_time: 7869126,
        total_score: 1160981443028,
        total_hits: 74253363,
        maximum_combo: 10032,
        replays_watched_by_others: 7488,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 2321,
            ssh: 8952,
            s: 2301,
            sh: 2398,
            a: 401
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/12863098?1745956940.jpeg",
            country: Some(Country {
                code: "FI",
                name: "Finland"
            }),
            country_code: "FI",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/12863098/4df5168668a396845d982d2debb9e402bfbc4dd705b9c1a11e6e8d28fff67085.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/12863098/4df5168668a396845d982d2debb9e402bfbc4dd705b9c1a11e6e8d28fff67085.png"
            }),
            default_group: "default",
            id: 12863098,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T04:26:30+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Veeti"
        })
    }, Statistics {
        count_100: 4738158,
        count_300: 79914412,
        count_50: 32404380,
        count_miss: 1778145,
        country_rank: None,
        level: Level {
            current: 112,
            progress: 52
        },
        global_rank: Some(60),
        global_rank_exp: None,
        pp: 18876.8,
        pp_exp: Some(0.0),
        ranked_score: 364955851319,
        hit_accuracy: 99.915,
        play_count: 246021,
        play_time: 14021058,
        total_score: 1279366274994,
        total_hits: 117056950,
        maximum_combo: 10032,
        replays_watched_by_others: 7756,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1423,
            ssh: 15477,
            s: 2051,
            sh: 1556,
            a: 174
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/6701104?1743538883.jpeg",
            country: Some(Country {
                code: "AU",
                name: "Australia"
            }),
            country_code: "AU",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/6701104/f2784d6f4b0625fa0d5d2e8d99e59041f1d86ed56fc67a445127022dff584454.png"),
                id: Some("1"),
                url: "https://assets.ppy.sh/user-cover-presets/1/df28696b58541a9e67f6755918951d542d93bdf1da41720fcca2fd2c1ea8cf51.jpeg"
            }),
            default_group: "default",
            id: 6701104,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T06:54:12+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Nene Sakura"
        })
    }, Statistics {
        count_100: 5326703,
        count_300: 88102654,
        count_50: 32406997,
        count_miss: 2017471,
        country_rank: None,
        level: Level {
            current: 112,
            progress: 80
        },
        global_rank: Some(173),
        global_rank_exp: None,
        pp: 15504.6,
        pp_exp: Some(0.0),
        ranked_score: 334621954819,
        hit_accuracy: 99.9261,
        play_count: 269966,
        play_time: 14641570,
        total_score: 1307188201722,
        total_hits: 125836354,
        maximum_combo: 9225,
        replays_watched_by_others: 13114,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 602,
            ssh: 12526,
            s: 1247,
            sh: 6385,
            a: 127
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/8417087?1745916392.jpeg",
            country: Some(Country {
                code: "UA",
                name: "Ukraine"
            }),
            country_code: "UA",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/8417087/cf0acedd08b3f8129e5966bdbcf6782d04976921aaadf15b3aeed03ecf6029b5.png"),
                id: Some("22"),
                url: "https://assets.ppy.sh/user-cover-presets/22/fe7e6dce2328b292d15b03ada3df83064ff58eb77f0ec6d97fb0d8193d8e7de2.jpeg"
            }),
            default_group: "default",
            id: 8417087,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-12T06:20:11+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "FruktoLove"
        })
    }, Statistics {
        count_100: 2172710,
        count_300: 42567448,
        count_50: 19741926,
        count_miss: 581286,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 62
        },
        global_rank: Some(573),
        global_rank_exp: None,
        pp: 10664.5,
        pp_exp: Some(0.0),
        ranked_score: 322974942135,
        hit_accuracy: 99.9755,
        play_count: 73680,
        play_time: 8704986,
        total_score: 589607576725,
        total_hits: 64482084,
        maximum_combo: 9225,
        replays_watched_by_others: 1155,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 11267,
            ssh: 11723,
            s: 2479,
            sh: 163,
            a: 264
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/15074740?1746625072.jpeg",
            country: Some(Country {
                code: "PL",
                name: "Poland"
            }),
            country_code: "PL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/15074740/ecd17e67bdac2a610d4daa6ed65d6253ccb734e9c96b4b1dc7c5f75992da0cd5.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/15074740/ecd17e67bdac2a610d4daa6ed65d6253ccb734e9c96b4b1dc7c5f75992da0cd5.png"
            }),
            default_group: "default",
            id: 15074740,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-12T03:53:26+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Arctaroll"
        })
    }, Statistics {
        count_100: 2132912,
        count_300: 67011032,
        count_50: 32288603,
        count_miss: 826066,
        country_rank: None,
        level: Level {
            current: 109,
            progress: 4
        },
        global_rank: Some(432),
        global_rank_exp: None,
        pp: 11908.1,
        pp_exp: Some(0.0),
        ranked_score: 310160856166,
        hit_accuracy: 99.8268,
        play_count: 173681,
        play_time: 11870064,
        total_score: 931732003385,
        total_hits: 101432547,
        maximum_combo: 5419,
        replays_watched_by_others: 3946,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 4159,
            ssh: 10559,
            s: 2255,
            sh: 2916,
            a: 0
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/2624179?1746890079.png",
            country: Some(Country {
                code: "JP",
                name: "Japan"
            }),
            country_code: "JP",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/2624179/f35304b9bcae75c2aa333a20656c312067729a682639c68b68e4cbb46212f221.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/2624179/f35304b9bcae75c2aa333a20656c312067729a682639c68b68e4cbb46212f221.png"
            }),
            default_group: "default",
            id: 2624179,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Melu"
        })
    }, Statistics {
        count_100: 3689354,
        count_300: 61861357,
        count_50: 27782191,
        count_miss: 3929125,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 81
        },
        global_rank: Some(917),
        global_rank_exp: None,
        pp: 8477.14,
        pp_exp: Some(0.0),
        ranked_score: 302983899656,
        hit_accuracy: 99.4268,
        play_count: 175104,
        play_time: 11476065,
        total_score: 608355610707,
        total_hits: 93332902,
        maximum_combo: 9225,
        replays_watched_by_others: 1628,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 24181,
            ssh: 1000,
            s: 6641,
            sh: 1956,
            a: 3483
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/16196079?1746140121.jpeg",
            country: Some(Country {
                code: "RO",
                name: "Romania"
            }),
            country_code: "RO",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/16196079/1ff6072274b52f9e8fa89f501498d7558fb445576d1ca991248c6fc5c0f10a3e.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/16196079/1ff6072274b52f9e8fa89f501498d7558fb445576d1ca991248c6fc5c0f10a3e.png"
            }),
            default_group: "default",
            id: 16196079,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T04:21:34+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Romania"
        })
    }, Statistics {
        count_100: 2547172,
        count_300: 47645921,
        count_50: 27560411,
        count_miss: 482959,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 46
        },
        global_rank: Some(1455),
        global_rank_exp: None,
        pp: 6757.43,
        pp_exp: Some(0.0),
        ranked_score: 292196968109,
        hit_accuracy: 99.7563,
        play_count: 167324,
        play_time: 11145042,
        total_score: 573273075870,
        total_hits: 77753504,
        maximum_combo: 9225,
        replays_watched_by_others: 2558,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 26561,
            ssh: 8894,
            s: 1593,
            sh: 318,
            a: 238
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/7552274?1745688109.jpeg",
            country: Some(Country {
                code: "PL",
                name: "Poland"
            }),
            country_code: "PL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/7552274/24403aa3a2cc3416594e16b7135fc234029cece2c4078f478114a09522b39ad0.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/7552274/24403aa3a2cc3416594e16b7135fc234029cece2c4078f478114a09522b39ad0.png"
            }),
            default_group: "default",
            id: 7552274,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: true,
            profile_colour: None,
            username: "-ExGon-"
        })
    }, Statistics {
        count_100: 1282332,
        count_300: 60648815,
        count_50: 23939054,
        count_miss: 657939,
        country_rank: None,
        level: Level {
            current: 106,
            progress: 92
        },
        global_rank: Some(1798),
        global_rank_exp: None,
        pp: 5882.87,
        pp_exp: Some(0.0),
        ranked_score: 288043454138,
        hit_accuracy: 99.773,
        play_count: 116188,
        play_time: 9435886,
        total_score: 719594858519,
        total_hits: 85870201,
        maximum_combo: 4257,
        replays_watched_by_others: 1011,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 5437,
            ssh: 1909,
            s: 8742,
            sh: 891,
            a: 139
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/5261202?1738597669.jpeg",
            country: Some(Country {
                code: "KR",
                name: "South Korea"
            }),
            country_code: "KR",
            cover: Some(Cover {
                custom_url: None,
                id: Some("29"),
                url: "https://assets.ppy.sh/user-cover-presets/29/fce0517b1879df78357e3fd54478afeda91ce9e6254642ee17f429e06389dace.jpeg"
            }),
            default_group: "default",
            id: 5261202,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Chii Aruel"
        })
    }, Statistics {
        count_100: 2952958,
        count_300: 44847874,
        count_50: 32712167,
        count_miss: 265093,
        country_rank: None,
        level: Level {
            current: 106,
            progress: 2
        },
        global_rank: Some(1565),
        global_rank_exp: None,
        pp: 6471.03,
        pp_exp: Some(0.0),
        ranked_score: 286497605644,
        hit_accuracy: 99.8652,
        play_count: 166089,
        play_time: 10184225,
        total_score: 629095951994,
        total_hits: 80512999,
        maximum_combo: 9225,
        replays_watched_by_others: 9649,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 36309,
            ssh: 13945,
            s: 812,
            sh: 1536,
            a: 41
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4297805?1641467126.png",
            country: Some(Country {
                code: "CN",
                name: "China"
            }),
            country_code: "CN",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4297805/ae22d91a7437124144cd02efbf2be82050ac9f499a68026a222a4bfd6eebfa45.jpeg"),
                id: Some("1"),
                url: "https://assets.ppy.sh/user-cover-presets/1/df28696b58541a9e67f6755918951d542d93bdf1da41720fcca2fd2c1ea8cf51.jpeg"
            }),
            default_group: "default",
            id: 4297805,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-10T02:20:59+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Edogawa Conan"
        })
    }, Statistics {
        count_100: 1721111,
        count_300: 43104573,
        count_50: 22942839,
        count_miss: 783431,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 61
        },
        global_rank: Some(1592),
        global_rank_exp: None,
        pp: 6407.97,
        pp_exp: Some(0.0),
        ranked_score: 282909763019,
        hit_accuracy: 99.6698,
        play_count: 119690,
        play_time: 8421329,
        total_score: 488616932194,
        total_hits: 67768523,
        maximum_combo: 7306,
        replays_watched_by_others: 786,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 17895,
            ssh: 7561,
            s: 3799,
            sh: 614,
            a: 174
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4577865?1738113771.gif",
            country: Some(Country {
                code: "PH",
                name: "Philippines"
            }),
            country_code: "PH",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4577865/de279582bf4e86ef588ff48da44a6dc3d16111a3b97e3953d28b73c18b18940f.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/4577865/de279582bf4e86ef588ff48da44a6dc3d16111a3b97e3953d28b73c18b18940f.jpeg"
            }),
            default_group: "default",
            id: 4577865,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-12T07:07:45+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Chloebe"
        })
    }, Statistics {
        count_100: 4165939,
        count_300: 83784505,
        count_50: 25783238,
        count_miss: 2477572,
        country_rank: None,
        level: Level {
            current: 110,
            progress: 85
        },
        global_rank: Some(55),
        global_rank_exp: None,
        pp: 19034.3,
        pp_exp: Some(0.0),
        ranked_score: 279643102377,
        hit_accuracy: 99.9151,
        play_count: 254109,
        play_time: 13646983,
        total_score: 1112744966317,
        total_hits: 113733682,
        maximum_combo: 9225,
        replays_watched_by_others: 151080,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 7177,
            ssh: 5302,
            s: 2049,
            sh: 265,
            a: 213
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4365562?1738028904.jpeg",
            country: Some(Country {
                code: "US",
                name: "United States"
            }),
            country_code: "US",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4365562/7b697ca0c725971bfacf894eb0f53c0986e3ce9231cce03de51b6aca98fea063.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/4365562/7b697ca0c725971bfacf894eb0f53c0986e3ce9231cce03de51b6aca98fea063.jpeg"
            }),
            default_group: "default",
            id: 4365562,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T01:35:53+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "rostld"
        })
    }, Statistics {
        count_100: 1948677,
        count_300: 33630194,
        count_50: 24817209,
        count_miss: 940861,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 1
        },
        global_rank: Some(1525),
        global_rank_exp: None,
        pp: 6561.6,
        pp_exp: Some(0.0),
        ranked_score: 269001226336,
        hit_accuracy: 99.7293,
        play_count: 132325,
        play_time: 7888823,
        total_score: 428199360245,
        total_hits: 60396080,
        maximum_combo: 9225,
        replays_watched_by_others: 1291,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 28967,
            ssh: 3110,
            s: 1161,
            sh: 16,
            a: 8
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/14477534?1745066159.png",
            country: Some(Country {
                code: "RS",
                name: "Serbia"
            }),
            country_code: "RS",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/14477534/2fcc433c1ba966077a7db9b627177517ba678d28f9502efb0d94effc23735d04.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/14477534/2fcc433c1ba966077a7db9b627177517ba678d28f9502efb0d94effc23735d04.jpeg"
            }),
            default_group: "default",
            id: 14477534,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T03:04:38+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "optm"
        })
    }, Statistics {
        count_100: 2859437,
        count_300: 49740016,
        count_50: 18547144,
        count_miss: 1244416,
        country_rank: None,
        level: Level {
            current: 106,
            progress: 93
        },
        global_rank: Some(37),
        global_rank_exp: None,
        pp: 19789.4,
        pp_exp: Some(0.0),
        ranked_score: 266359247846,
        hit_accuracy: 99.9195,
        play_count: 137141,
        play_time: 7672287,
        total_score: 720551339431,
        total_hits: 71146597,
        maximum_combo: 9225,
        replays_watched_by_others: 2513,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 4841,
            ssh: 8733,
            s: 1960,
            sh: 2488,
            a: 494
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/14571181?1735888617.png",
            country: Some(Country {
                code: "CA",
                name: "Canada"
            }),
            country_code: "CA",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/14571181/4c0cb38c160ade49db600327595a60430203fae9f2dd2377111c470299a0e632.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/14571181/4c0cb38c160ade49db600327595a60430203fae9f2dd2377111c470299a0e632.jpeg"
            }),
            default_group: "default",
            id: 14571181,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-11T04:05:50+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "sularis"
        })
    }, Statistics {
        count_100: 1748566,
        count_300: 25092182,
        count_50: 21800540,
        count_miss: 113517,
        country_rank: None,
        level: Level {
            current: 102,
            progress: 76
        },
        global_rank: Some(1333),
        global_rank_exp: None,
        pp: 7092.95,
        pp_exp: Some(0.0),
        ranked_score: 263311648332,
        hit_accuracy: 100.0,
        play_count: 71989,
        play_time: 7332717,
        total_score: 303150940844,
        total_hits: 48641288,
        maximum_combo: 5908,
        replays_watched_by_others: 998,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 55200,
            ssh: 0,
            s: 0,
            sh: 0,
            a: 0
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/3774554?1746451964.png",
            country: Some(Country {
                code: "FR",
                name: "France"
            }),
            country_code: "FR",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/3774554/807d3071f15a205705f2a5de35ceb4eb3490a932d303d50db796fd4db7ff0a6d.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/3774554/807d3071f15a205705f2a5de35ceb4eb3490a932d303d50db796fd4db7ff0a6d.png"
            }),
            default_group: "default",
            id: 3774554,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: true,
            profile_colour: None,
            username: "Axiaan"
        })
    }, Statistics {
        count_100: 3346200,
        count_300: 38098668,
        count_50: 35710565,
        count_miss: 76207,
        country_rank: None,
        level: Level {
            current: 103,
            progress: 65
        },
        global_rank: Some(1121),
        global_rank_exp: None,
        pp: 7808.45,
        pp_exp: Some(0.0),
        ranked_score: 253852231162,
        hit_accuracy: 100.0,
        play_count: 139811,
        play_time: 9901154,
        total_score: 392441860563,
        total_hits: 77155433,
        maximum_combo: 5908,
        replays_watched_by_others: 487,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 64275,
            ssh: 0,
            s: 0,
            sh: 0,
            a: 0
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/5070783?1648908103.png",
            country: Some(Country {
                code: "AX",
                name: "Aland Islands"
            }),
            country_code: "AX",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/5070783/92e1ab847ec095969da42ad247956e9fb786f1147f7573865c19278ea10283c6.gif"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/5070783/92e1ab847ec095969da42ad247956e9fb786f1147f7573865c19278ea10283c6.gif"
            }),
            default_group: "default",
            id: 5070783,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T21:20:34+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Osdare"
        })
    }, Statistics {
        count_100: 1709013,
        count_300: 31158260,
        count_50: 10081448,
        count_miss: 536336,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 53
        },
        global_rank: Some(76),
        global_rank_exp: None,
        pp: 18301.0,
        pp_exp: Some(0.0),
        ranked_score: 252055983220,
        hit_accuracy: 99.9376,
        play_count: 61604,
        play_time: 4788679,
        total_score: 580746530129,
        total_hits: 42948721,
        maximum_combo: 10032,
        replays_watched_by_others: 4134,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 7422,
            ssh: 429,
            s: 3330,
            sh: 302,
            a: 123
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4741164?1744638096.jpeg",
            country: Some(Country {
                code: "RU",
                name: "Russian Federation"
            }),
            country_code: "RU",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4741164/bc7088b053de2d550a8f1a77b63820df63cb40480eadcbdc1df4110ecdc85342.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/4741164/bc7088b053de2d550a8f1a77b63820df63cb40480eadcbdc1df4110ecdc85342.jpeg"
            }),
            default_group: "bng",
            id: 4741164,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: Some("#A347EB"),
            username: "Nelly"
        })
    }, Statistics {
        count_100: 6194696,
        count_300: 58208617,
        count_50: 44407499,
        count_miss: 429341,
        country_rank: None,
        level: Level {
            current: 107,
            progress: 21
        },
        global_rank: Some(623),
        global_rank_exp: None,
        pp: 10338.8,
        pp_exp: Some(0.0),
        ranked_score: 245187689450,
        hit_accuracy: 99.9984,
        play_count: 257692,
        play_time: 13364479,
        total_score: 748904937310,
        total_hits: 108810812,
        maximum_combo: 9225,
        replays_watched_by_others: 214510,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 10390,
            ssh: 18162,
            s: 312,
            sh: 459,
            a: 28
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/1459769?1544728288.png",
            country: Some(Country {
                code: "KR",
                name: "South Korea"
            }),
            country_code: "KR",
            cover: Some(Cover {
                custom_url: None,
                id: Some("2"),
                url: "https://assets.ppy.sh/user-cover-presets/2/f5142b64b60002f6314b22c775195950105908e149037f4de78efc0e0f28d442.jpeg"
            }),
            default_group: "default",
            id: 1459769,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-11T23:24:07+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "mjj741"
        })
    }, Statistics {
        count_100: 2715801,
        count_300: 50266660,
        count_50: 22862903,
        count_miss: 1299158,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 40
        },
        global_rank: Some(914),
        global_rank_exp: None,
        pp: 8478.97,
        pp_exp: Some(0.0),
        ranked_score: 243236799786,
        hit_accuracy: 99.5206,
        play_count: 144789,
        play_time: 10076251,
        total_score: 467008657764,
        total_hits: 75845364,
        maximum_combo: 6699,
        replays_watched_by_others: 9140,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 2429,
            ssh: 5135,
            s: 5704,
            sh: 5652,
            a: 849
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/3716061?1719736209.png",
            country: Some(Country {
                code: "PL",
                name: "Poland"
            }),
            country_code: "PL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/3716061/86feb0e50d1fb3883390d7a5e84ad03ee844c1aaa497f8d1b64873ec3a7487f4.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/3716061/86feb0e50d1fb3883390d7a5e84ad03ee844c1aaa497f8d1b64873ec3a7487f4.jpeg"
            }),
            default_group: "default",
            id: 3716061,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-07T09:35:49+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Rawaj"
        })
    }, Statistics {
        count_100: 3980435,
        count_300: 51531791,
        count_50: 38533298,
        count_miss: 1031623,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 14
        },
        global_rank: Some(361),
        global_rank_exp: None,
        pp: 12638.9,
        pp_exp: Some(0.0),
        ranked_score: 242276576842,
        hit_accuracy: 99.9835,
        play_count: 262393,
        play_time: 11736214,
        total_score: 541480822871,
        total_hits: 94045524,
        maximum_combo: 7750,
        replays_watched_by_others: 14955,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 19384,
            ssh: 29907,
            s: 605,
            sh: 10,
            a: 59
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/9114235?1679136268.jpeg",
            country: Some(Country {
                code: "NL",
                name: "Netherlands"
            }),
            country_code: "NL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/9114235/0cd76f11af673b88687d4b6afc6eed100b9f7f121e10219111b728e8864a1cce.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/9114235/0cd76f11af673b88687d4b6afc6eed100b9f7f121e10219111b728e8864a1cce.jpeg"
            }),
            default_group: "default",
            id: 9114235,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "KevKjef"
        })
    }, Statistics {
        count_100: 1393475,
        count_300: 28360772,
        count_50: 15227378,
        count_miss: 627666,
        country_rank: None,
        level: Level {
            current: 102,
            progress: 97
        },
        global_rank: Some(1103),
        global_rank_exp: None,
        pp: 7894.9,
        pp_exp: Some(0.0),
        ranked_score: 238258309914,
        hit_accuracy: 99.4857,
        play_count: 76376,
        play_time: 5631030,
        total_score: 324774931231,
        total_hits: 44981625,
        maximum_combo: 5165,
        replays_watched_by_others: 412,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 10137,
            ssh: 183,
            s: 16974,
            sh: 394,
            a: 2486
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/7807460?1707683588.jpeg",
            country: Some(Country {
                code: "US",
                name: "United States"
            }),
            country_code: "US",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/7807460/02f44761e70d5391ca76782c35e1b6ed5d17850ab62e8495e7a7608bcdff4c9f.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/7807460/02f44761e70d5391ca76782c35e1b6ed5d17850ab62e8495e7a7608bcdff4c9f.jpeg"
            }),
            default_group: "default",
            id: 7807460,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-10T04:21:04+00:00"),
            pm_friends_only: true,
            profile_colour: None,
            username: "shinitaichan"
        })
    }, Statistics {
        count_100: 2548477,
        count_300: 44304312,
        count_50: 14049766,
        count_miss: 947718,
        country_rank: None,
        level: Level {
            current: 107,
            progress: 27
        },
        global_rank: Some(115),
        global_rank_exp: None,
        pp: 16983.1,
        pp_exp: Some(0.0),
        ranked_score: 236897423807,
        hit_accuracy: 99.9457,
        play_count: 126599,
        play_time: 7096147,
        total_score: 754248526346,
        total_hits: 60902555,
        maximum_combo: 9281,
        replays_watched_by_others: 913,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 7406,
            ssh: 643,
            s: 5502,
            sh: 439,
            a: 326
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/4586304?1745520087.jpeg",
            country: Some(Country {
                code: "NO",
                name: "Norway"
            }),
            country_code: "NO",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/4586304/aa1c911710bf350df3026afeb7d4f44065a503ff5da5ac176cf4fba6ab5be860.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/4586304/aa1c911710bf350df3026afeb7d4f44065a503ff5da5ac176cf4fba6ab5be860.jpeg"
            }),
            default_group: "default",
            id: 4586304,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T20:13:33+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "CyCeph"
        })
    }, Statistics {
        count_100: 2000555,
        count_300: 41413153,
        count_50: 11309183,
        count_miss: 1164415,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 21
        },
        global_rank: Some(48),
        global_rank_exp: None,
        pp: 19239.0,
        pp_exp: Some(0.0),
        ranked_score: 233453460657,
        hit_accuracy: 99.9733,
        play_count: 100371,
        play_time: 5989879,
        total_score: 548687701753,
        total_hits: 54722891,
        maximum_combo: 7750,
        replays_watched_by_others: 1569,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 6417,
            ssh: 605,
            s: 6321,
            sh: 702,
            a: 625
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/7024526?1731144631.jpeg",
            country: Some(Country {
                code: "FI",
                name: "Finland"
            }),
            country_code: "FI",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/7024526/fbe970c9112fd9b00982f157489f7a360ea2726f040108195c7bc045d06aad23.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/7024526/fbe970c9112fd9b00982f157489f7a360ea2726f040108195c7bc045d06aad23.png"
            }),
            default_group: "default",
            id: 7024526,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T23:00:01+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Leinou"
        })
    }, Statistics {
        count_100: 1881008,
        count_300: 31528732,
        count_50: 11978599,
        count_miss: 750499,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 13
        },
        global_rank: Some(75),
        global_rank_exp: None,
        pp: 18351.6,
        pp_exp: Some(0.0),
        ranked_score: 232510001699,
        hit_accuracy: 99.9902,
        play_count: 108844,
        play_time: 5347909,
        total_score: 440639425000,
        total_hits: 45388339,
        maximum_combo: 8399,
        replays_watched_by_others: 3355,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 6229,
            ssh: 6292,
            s: 1830,
            sh: 471,
            a: 1198
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/16352736?1737928096.png",
            country: Some(Country {
                code: "GB",
                name: "United Kingdom"
            }),
            country_code: "GB",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/16352736/df43df89992624b58fd7fb009c6466d45f3d0e8dc47f5ca86661176af4cf1a59.jpeg"),
                id: Some("4"),
                url: "https://assets.ppy.sh/user-cover-presets/4/2fd772ad175c5687370e0aab50799a84adef7d0fff3f97dccfa5c94384ebb8af.jpeg"
            }),
            default_group: "default",
            id: 16352736,
            is_active: false,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-01-26T22:20:02+00:00"),
            pm_friends_only: true,
            profile_colour: None,
            username: "BIG H ZONDA KIT"
        })
    }, Statistics {
        count_100: 1040636,
        count_300: 31088283,
        count_50: 20380067,
        count_miss: 285262,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 15
        },
        global_rank: Some(576),
        global_rank_exp: None,
        pp: 10635.6,
        pp_exp: Some(0.0),
        ranked_score: 228129375384,
        hit_accuracy: 99.9998,
        play_count: 106039,
        play_time: 6438626,
        total_score: 442915735849,
        total_hits: 52508986,
        maximum_combo: 5908,
        replays_watched_by_others: 8372,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 6541,
            ssh: 21079,
            s: 1,
            sh: 26,
            a: 0
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/17838855?1741736352.png",
            country: Some(Country {
                code: "TH",
                name: "Thailand"
            }),
            country_code: "TH",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/17838855/040c4d93a5ad1020f646953b4be46c2976e732f2547e1206fd82259707adf5b9.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/17838855/040c4d93a5ad1020f646953b4be46c2976e732f2547e1206fd82259707adf5b9.jpeg"
            }),
            default_group: "default",
            id: 17838855,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Char0n"
        })
    }, Statistics {
        count_100: 1024633,
        count_300: 28010731,
        count_50: 17522278,
        count_miss: 193707,
        country_rank: None,
        level: Level {
            current: 103,
            progress: 42
        },
        global_rank: Some(1597),
        global_rank_exp: None,
        pp: 6397.8,
        pp_exp: Some(0.0),
        ranked_score: 222066070046,
        hit_accuracy: 99.5799,
        play_count: 100908,
        play_time: 5837389,
        total_score: 369821679452,
        total_hits: 46557642,
        maximum_combo: 9225,
        replays_watched_by_others: 1189,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 5696,
            ssh: 13717,
            s: 1778,
            sh: 2296,
            a: 262
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/9096327?1747011276.png",
            country: Some(Country {
                code: "CO",
                name: "Colombia"
            }),
            country_code: "CO",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/9096327/e8e1b091b0972b1423b0b1620f62eb72d373b8f97675d86b0f5873b33db857e1.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/9096327/e8e1b091b0972b1423b0b1620f62eb72d373b8f97675d86b0f5873b33db857e1.jpeg"
            }),
            default_group: "default",
            id: 9096327,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T00:52:03+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Masaru"
        })
    }, Statistics {
        count_100: 2602685,
        count_300: 69262778,
        count_50: 17672537,
        count_miss: 1136911,
        country_rank: None,
        level: Level {
            current: 110,
            progress: 60
        },
        global_rank: Some(9),
        global_rank_exp: None,
        pp: 23683.6,
        pp_exp: Some(0.0),
        ranked_score: 221435766211,
        hit_accuracy: 99.9674,
        play_count: 180812,
        play_time: 10930581,
        total_score: 1087650554376,
        total_hits: 89538000,
        maximum_combo: 9281,
        replays_watched_by_others: 47662,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1140,
            ssh: 2868,
            s: 827,
            sh: 1190,
            a: 181
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/7226149?1740959611.png",
            country: Some(Country {
                code: "US",
                name: "United States"
            }),
            country_code: "US",
            cover: Some(Cover {
                custom_url: None,
                id: Some("8"),
                url: "https://assets.ppy.sh/user-cover-presets/8/eda177e3c4eb0889e24f58c454af47708b0acd612bff984f6ac295e8427681c5.jpeg"
            }),
            default_group: "default",
            id: 7226149,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-12T05:48:32+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Lexii"
        })
    }, Statistics {
        count_100: 3084831,
        count_300: 29356593,
        count_50: 28882167,
        count_miss: 98321,
        country_rank: None,
        level: Level {
            current: 103,
            progress: 39
        },
        global_rank: Some(682),
        global_rank_exp: None,
        pp: 9955.72,
        pp_exp: Some(0.0),
        ranked_score: 220980078202,
        hit_accuracy: 100.0,
        play_count: 107175,
        play_time: 8114728,
        total_score: 366369683376,
        total_hits: 61323591,
        maximum_combo: 9225,
        replays_watched_by_others: 400,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 48197,
            ssh: 31,
            s: 137,
            sh: -1,
            a: 37
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/8805840?1744651868.jpeg",
            country: Some(Country {
                code: "NL",
                name: "Netherlands"
            }),
            country_code: "NL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/8805840/a5264ee63cb19c8aec7af574c7c5c10072c012b85dbe7ce1855d9763b82be53a.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/8805840/a5264ee63cb19c8aec7af574c7c5c10072c012b85dbe7ce1855d9763b82be53a.jpeg"
            }),
            default_group: "default",
            id: 8805840,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Aefkay"
        })
    }, Statistics {
        count_100: 2600860,
        count_300: 38759621,
        count_50: 12677779,
        count_miss: 813385,
        country_rank: None,
        level: Level {
            current: 106,
            progress: 40
        },
        global_rank: Some(212),
        global_rank_exp: None,
        pp: 14801.7,
        pp_exp: Some(0.0),
        ranked_score: 220785141437,
        hit_accuracy: 99.9196,
        play_count: 92027,
        play_time: 6208910,
        total_score: 667295684684,
        total_hits: 54038260,
        maximum_combo: 7259,
        replays_watched_by_others: 268,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 4097,
            ssh: 1749,
            s: 3096,
            sh: 2654,
            a: 186
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/11152298?1581457310.jpeg",
            country: Some(Country {
                code: "FR",
                name: "France"
            }),
            country_code: "FR",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/11152298/580a6c9536c57db8edb4722659e9aea36dfddfc541f585c233420cf76702319a.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/11152298/580a6c9536c57db8edb4722659e9aea36dfddfc541f585c233420cf76702319a.png"
            }),
            default_group: "default",
            id: 11152298,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T22:58:42+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Airion"
        })
    }, Statistics {
        count_100: 4478295,
        count_300: 67685783,
        count_50: 23409232,
        count_miss: 1450554,
        country_rank: None,
        level: Level {
            current: 112,
            progress: 23
        },
        global_rank: Some(186),
        global_rank_exp: None,
        pp: 15322.8,
        pp_exp: Some(0.0),
        ranked_score: 216792095564,
        hit_accuracy: 99.921,
        play_count: 172406,
        play_time: 10900162,
        total_score: 1250418615450,
        total_hits: 95573310,
        maximum_combo: 9281,
        replays_watched_by_others: 3801,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1576,
            ssh: 5475,
            s: 1932,
            sh: 2276,
            a: 43
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/6508754?1641329765.jpeg",
            country: Some(Country {
                code: "US",
                name: "United States"
            }),
            country_code: "US",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/6508754/c25dbaf9a38c83f2566427f4416975c8cf200b28d67076b206a9beb6ef9b3cc2.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/6508754/c25dbaf9a38c83f2566427f4416975c8cf200b28d67076b206a9beb6ef9b3cc2.jpeg"
            }),
            default_group: "default",
            id: 6508754,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T18:57:53+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Edgar_Figaro"
        })
    }, Statistics {
        count_100: 5885236,
        count_300: 117079845,
        count_50: 26584282,
        count_miss: 2268539,
        country_rank: None,
        level: Level {
            current: 118,
            progress: 98
        },
        global_rank: Some(1),
        global_rank_exp: None,
        pp: 26698.8,
        pp_exp: Some(0.0),
        ranked_score: 214717145067,
        hit_accuracy: 99.9652,
        play_count: 308800,
        play_time: 16274962,
        total_score: 1925695990920,
        total_hits: 149549363,
        maximum_combo: 9281,
        replays_watched_by_others: 39438,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1545,
            ssh: 156,
            s: 3128,
            sh: 171,
            a: 252
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/7547506?1731666368.png",
            country: Some(Country {
                code: "KR",
                name: "South Korea"
            }),
            country_code: "KR",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/7547506/b06978f66babb814e8bad8fb41beff215d6d92da9fc868bcb79e949440a942fa.jpeg"),
                id: Some("3"),
                url: "https://assets.ppy.sh/user-cover-presets/3/32ddb3eb261e38a82067f9ef4ea96c12f6abf8bd228e6413330f9d351420301b.jpeg"
            }),
            default_group: "default",
            id: 7547506,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "qwhj79"
        })
    }, Statistics {
        count_100: 2734454,
        count_300: 84205554,
        count_50: 41181351,
        count_miss: 1078997,
        country_rank: None,
        level: Level {
            current: 109,
            progress: 65
        },
        global_rank: Some(703),
        global_rank_exp: None,
        pp: 9818.57,
        pp_exp: Some(0.0),
        ranked_score: 213314986636,
        hit_accuracy: 99.8161,
        play_count: 371477,
        play_time: 15806969,
        total_score: 992209096609,
        total_hits: 128121359,
        maximum_combo: 5419,
        replays_watched_by_others: 4441,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 1336,
            ssh: 13414,
            s: 342,
            sh: 865,
            a: 160
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/3538446?1731015837.jpeg",
            country: Some(Country {
                code: "TW",
                name: "Taiwan"
            }),
            country_code: "TW",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/3538446/7aa3bf181ed2a278315fbeb8e670e42f9ec7b1ab064f31fb6746f1321fdf6f17.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/3538446/7aa3bf181ed2a278315fbeb8e670e42f9ec7b1ab064f31fb6746f1321fdf6f17.jpeg"
            }),
            default_group: "default",
            id: 3538446,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: None,
            pm_friends_only: false,
            profile_colour: None,
            username: "Tomari Mari"
        })
    }, Statistics {
        count_100: 2376847,
        count_300: 42798413,
        count_50: 24441798,
        count_miss: 559617,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 89
        },
        global_rank: Some(939),
        global_rank_exp: None,
        pp: 8418.38,
        pp_exp: Some(0.0),
        ranked_score: 210352207843,
        hit_accuracy: 99.9906,
        play_count: 162820,
        play_time: 8566042,
        total_score: 516113385006,
        total_hits: 69617058,
        maximum_combo: 7383,
        replays_watched_by_others: 11240,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 5056,
            ssh: 9012,
            s: 1074,
            sh: 2630,
            a: 155
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/2173232?1736348399.jpeg",
            country: Some(Country {
                code: "PE",
                name: "Peru"
            }),
            country_code: "PE",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/2173232/23d4b4ef6aba43172eb79c6eb1bab646b9ee038ef92f71356b541d56a82bb4d1.jpeg"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/2173232/23d4b4ef6aba43172eb79c6eb1bab646b9ee038ef92f71356b541d56a82bb4d1.jpeg"
            }),
            default_group: "default",
            id: 2173232,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-12T05:27:52+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Didfus"
        })
    }, Statistics {
        count_100: 2084295,
        count_300: 45441588,
        count_50: 16552966,
        count_miss: 1483820,
        country_rank: None,
        level: Level {
            current: 105,
            progress: 34
        },
        global_rank: Some(251),
        global_rank_exp: None,
        pp: 14145.5,
        pp_exp: Some(0.0),
        ranked_score: 207470634481,
        hit_accuracy: 99.907,
        play_count: 121198,
        play_time: 7815824,
        total_score: 561423784628,
        total_hits: 64078849,
        maximum_combo: 9225,
        replays_watched_by_others: 3789,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 9407,
            ssh: 1298,
            s: 3916,
            sh: 883,
            a: 368
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/5124924?1745890474.png",
            country: Some(Country {
                code: "CL",
                name: "Chile"
            }),
            country_code: "CL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/5124924/646b916093f8c44b1490743781aa808295c6e447e99ae98fd84c47775b61b81a.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/5124924/646b916093f8c44b1490743781aa808295c6e447e99ae98fd84c47775b61b81a.png"
            }),
            default_group: "default",
            id: 5124924,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T20:12:35+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "Ritmo Brigido"
        })
    }, Statistics {
        count_100: 4067547,
        count_300: 44068959,
        count_50: 31709587,
        count_miss: 664236,
        country_rank: None,
        level: Level {
            current: 104,
            progress: 1
        },
        global_rank: Some(1083),
        global_rank_exp: None,
        pp: 7980.54,
        pp_exp: Some(0.0),
        ranked_score: 207450591791,
        hit_accuracy: 99.1196,
        play_count: 206866,
        play_time: 11644524,
        total_score: 428973398576,
        total_hits: 79846093,
        maximum_combo: 7079,
        replays_watched_by_others: 1219,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 15554,
            ssh: 10476,
            s: 1225,
            sh: 2774,
            a: 158
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://osu.ppy.sh/images/layout/avatar-guest@2x.png",
            country: Some(Country {
                code: "RU",
                name: "Russian Federation"
            }),
            country_code: "RU",
            cover: Some(Cover {
                custom_url: None,
                id: Some("1"),
                url: "https://assets.ppy.sh/user-cover-presets/1/df28696b58541a9e67f6755918951d542d93bdf1da41720fcca2fd2c1ea8cf51.jpeg"
            }),
            default_group: "default",
            id: 963369,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-11T00:15:25+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "IddaqJedd"
        })
    }, Statistics {
        count_100: 1312344,
        count_300: 21521051,
        count_50: 7781485,
        count_miss: 488856,
        country_rank: None,
        level: Level {
            current: 103,
            progress: 54
        },
        global_rank: Some(107),
        global_rank_exp: None,
        pp: 17262.8,
        pp_exp: Some(0.0),
        ranked_score: 203839808620,
        hit_accuracy: 99.8982,
        play_count: 47633,
        play_time: 3297646,
        total_score: 381516291003,
        total_hits: 30614880,
        maximum_combo: 7750,
        replays_watched_by_others: 685,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 8449,
            ssh: 32,
            s: 3557,
            sh: 16,
            a: 194
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/2957534?1733085566.jpeg",
            country: Some(Country {
                code: "PL",
                name: "Poland"
            }),
            country_code: "PL",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/2957534/c8ab63fe72154f65c502fc493be6896b9c09a90d8cbff59f873a5de3b8179cdb.gif"),
                id: Some("12"),
                url: "https://assets.ppy.sh/user-cover-presets/12/6e8d3402c8080c2d9549a98321e1bff111dd9c94603ccdb237597479cab6e8a7.jpeg"
            }),
            default_group: "default",
            id: 2957534,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: true,
            last_visit: Some("2025-05-11T22:42:00+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "madcin"
        })
    }, Statistics {
        count_100: 1480590,
        count_300: 25862654,
        count_50: 13041895,
        count_miss: 275054,
        country_rank: None,
        level: Level {
            current: 102,
            progress: 99
        },
        global_rank: Some(1947),
        global_rank_exp: None,
        pp: 5550.27,
        pp_exp: Some(0.0),
        ranked_score: 199285804230,
        hit_accuracy: 99.9987,
        play_count: 86168,
        play_time: 5022164,
        total_score: 326204833186,
        total_hits: 40385139,
        maximum_combo: 5908,
        replays_watched_by_others: 2890,
        rank_change_since_30_days: None,
        is_ranked: true,
        grade_counts: GradeCounts {
            ss: 14468,
            ssh: 2886,
            s: 6,
            sh: 0,
            a: 2
        },
        rank: None,
        variants: None,
        user: Some(User {
            avatar_url: "https://a.ppy.sh/653016?1676216977.png",
            country: Some(Country {
                code: "KR",
                name: "South Korea"
            }),
            country_code: "KR",
            cover: Some(Cover {
                custom_url: Some("https://assets.ppy.sh/user-profile-covers/653016/3dd2bb705e3bac70706a90b5ff8211b5b710aaa6bdf4f53a082b156bcd0a6cd3.png"),
                id: None,
                url: "https://assets.ppy.sh/user-profile-covers/653016/3dd2bb705e3bac70706a90b5ff8211b5b710aaa6bdf4f53a082b156bcd0a6cd3.png"
            }),
            default_group: "default",
            id: 653016,
            is_active: true,
            is_bot: false,
            is_online: false,
            is_supporter: false,
            last_visit: Some("2025-05-11T15:31:44+00:00"),
            pm_friends_only: false,
            profile_colour: None,
            username: "gaten"
        })
    }],
    spotlight: None,
    total: 10000
}
*/
