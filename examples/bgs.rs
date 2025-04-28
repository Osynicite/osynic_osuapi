// Beatmap Get Beatmaps
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::beatmaps::IBeatmaps;
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
    let beatmaps = client
        .beatmaps
        .get_beatmaps([5000433, 4457446].to_vec())
        .await?;
    println!("{:?}", beatmaps);

    Ok(())
}

/*
ReqwestBeatmaps get_Beatmaps
Beatmaps {
    beatmaps: [Beatmap {
        beatmapset_id: 2105932,
        difficulty_rating: 5.64,
        id: 4457446,
        mode: Osu,
        status: Ranked,
        total_length: 89,
        user_id: 12895865,
        version: "ririn's Extra",
        accuracy: 8.6,
        ar: 9.0,
        bpm: 174.0,
        convert: false,
        count_circles: 181,
        count_sliders: 171,
        count_spinners: 0,
        cs: 4.0,
        deleted_at: None,
        drain: 6.0,
        hit_length: 79,
        is_scoreable: true,
        last_updated: "2024-06-28T17:19:07Z",
        mode_int: 0,
        passcount: 153,
        playcount: 1477,
        ranked: 1,
        url: "https://osu.ppy.sh/beatmaps/4457446",
        checksum: "644d1822eacb717873270704f35d3ae6",
        max_combo: 530,
        beatmapset: Some(Beatmapset {
            artist: "Yanagi Mami",
            artist_unicode: "柳麻美",
            covers: Covers {
                cover: "https://assets.ppy.sh/beatmaps/2105932/covers/cover.jpg?1719595162",
                cover_2x: "https://assets.ppy.sh/beatmaps/2105932/covers/cover@2x.jpg?1719595162",
                card: "https://assets.ppy.sh/beatmaps/2105932/covers/card.jpg?1719595162",
                card_2x: "https://assets.ppy.sh/beatmaps/2105932/covers/card@2x.jpg?1719595162",
                list: "https://assets.ppy.sh/beatmaps/2105932/covers/list.jpg?1719595162",
                list_2x: "https://assets.ppy.sh/beatmaps/2105932/covers/list@2x.jpg?1719595162",
                slimcover: "https://assets.ppy.sh/beatmaps/2105932/covers/slimcover.jpg?1719595162",
                slimcover_2x: "https://assets.ppy.sh/beatmaps/2105932/covers/slimcover@2x.jpg?1719595162"
            },
            creator: "Aakki",
            favourite_count: 25,
            hype: None,
            id: 2105932,
            nsfw: false,
            offset: 0,
            play_count: 20940,
            preview_url: "//b.ppy.sh/preview/2105932.mp3",
            source: "ATRI -My Dear Moments-",
            spotlight: false,
            status: Ranked,
            title: "Hikari Hanate! (Short Ver.)",
            title_unicode: "光放て！(Short Ver.)",
            track_id: None,
            user_id: 11077540,
            video: false,
            bpm: 155.0,
            can_be_hyped: false,
            deleted_at: None,
            discussion_enabled: true,
            discussion_locked: false,
            is_scoreable: true,
            last_updated: "2024-06-28T17:19:05Z",
            legacy_thread_url: "https://osu.ppy.sh/community/forums/topics/1859201",
            nominations_summary: NominationsSummary {
                current: 2,
                eligible_main_rulesets: ["osu"],
                required_meta: RequiredMeta {
                    main_ruleset: 2,
                    non_main_ruleset: 1
                }
            },
            ranked: 1,
            ranked_date: "2024-07-01T04:03:39Z",
            storyboard: false,
            submitted_date: "2023-12-19T12:13:44Z",
            tags: "finnish bn lmt hanakumo rin dog food door weapon xiced een persoon butylcyclobutyn enneya アトリ makura 枕 frontwing フロントウイング video game vn すかぢ szak sca-ji fuminori matsumoto 松本文紀 japanese pop opening op short version galgame visual novel",
            availability: Availability {
                download_disabled: false,
                more_information: None
            },
            beatmaps: None,
            pack_tags: None,
            converts: None,
            current_nominations: None,
            description: None,
            genre: None,
            language: None,
            ratings: Some([0, 9, 0, 0, 1, 3, 1, 1, 2, 3, 32]),
            recent_favourites: None,
            related_users: None,
            related_tags: None,
            user: None
        }),
        current_user_tag_ids: None,
        failtimes: Some(Failtimes {
            fail: [0, 0, 0, 0, 0, 0, 0, 36, 144, 63, 9, 0, 0, 9, 54, 36, 0, 9, 9, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 9, 18, 27, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            exit: [9, 0, 0, 0, 0, 0, 0, 0, 81, 63, 36, 0, 0, 9, 18, 18, 144, 27, 9, 0, 0, 63, 45, 45, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }),
        owners: Some([Owner {
            id: 12895865,
            username: "Hanakumo Rin"
        }]),
        top_tag_ids: None
    }, Beatmap {
        beatmapset_id: 2331063,
        difficulty_rating: 6.4,
        id: 5000433,
        mode: Osu,
        status: Ranked,
        total_length: 135,
        user_id: 11119539,
        version: ":-b",
        accuracy: 9.0,
        ar: 9.3,
        bpm: 205.0,
        convert: false,
        count_circles: 380,
        count_sliders: 245,
        count_spinners: 0,
        cs: 4.0,
        deleted_at: None,
        drain: 6.0,
        hit_length: 135,
        is_scoreable: true,
        last_updated: "2025-02-27T18:56:28Z",
        mode_int: 0,
        passcount: 199,
        playcount: 1108,
        ranked: 1,
        url: "https://osu.ppy.sh/beatmaps/5000433",
        checksum: "57a9a80287c7abecc49a1bec77676ed9",
        max_combo: 890,
        beatmapset: Some(Beatmapset {
            artist: "DECO*27",
            artist_unicode: "DECO*27",
            covers: Covers {
                cover: "https://assets.ppy.sh/beatmaps/2331063/covers/cover.jpg?1740682609",
                cover_2x: "https://assets.ppy.sh/beatmaps/2331063/covers/cover@2x.jpg?1740682609",
                card: "https://assets.ppy.sh/beatmaps/2331063/covers/card.jpg?1740682609",
                card_2x: "https://assets.ppy.sh/beatmaps/2331063/covers/card@2x.jpg?1740682609",
                list: "https://assets.ppy.sh/beatmaps/2331063/covers/list.jpg?1740682609",
                list_2x: "https://assets.ppy.sh/beatmaps/2331063/covers/list@2x.jpg?1740682609",
                slimcover: "https://assets.ppy.sh/beatmaps/2331063/covers/slimcover.jpg?1740682609",
                slimcover_2x: "https://assets.ppy.sh/beatmaps/2331063/covers/slimcover@2x.jpg?1740682609"
            },
            creator: "AirinCat",
            favourite_count: 43,
            hype: None,
            id: 2331063,
            nsfw: false,
            offset: 0,
            play_count: 4734,
            preview_url: "//b.ppy.sh/preview/2331063.mp3",
            source: "",
            spotlight: false,
            status: Ranked,
            title: "Telepathy feat. Hatsune Miku",
            title_unicode: "テレパシ feat. 初音ミク",
            track_id: None,
            user_id: 11119539,
            video: true,
            bpm: 205.0,
            can_be_hyped: false,
            deleted_at: None,
            discussion_enabled: true,
            discussion_locked: false,
            is_scoreable: true,
            last_updated: "2025-02-27T18:56:27Z",
            legacy_thread_url: "https://osu.ppy.sh/community/forums/topics/2047151",
            nominations_summary: NominationsSummary {
                current: 2,
                eligible_main_rulesets: ["osu"],
                required_meta: RequiredMeta {
                    main_ruleset: 2,
                    non_main_ruleset: 1
                }
            },
            ranked: 1,
            ranked_date: "2025-03-06T19:05:47Z",
            storyboard: false,
            submitted_date: "2025-02-27T11:09:35Z",
            tags: "deco27 deco 27 vocaloid japanese pop electronic jpop j-pop otoiro tepe shota shouta horie kemu 堀江晶太 hibana ヒバナ dragon ball ドラゴンボール memes meme kyouren kittyadventure gokugohan12468 kensuke データなんかねえよ data nanka nee yo there's no data data*74 there is 提供クレジット teikyou sponsor credit explosions denpa",
            availability: Availability {
                download_disabled: false,
                more_information: None
            },
            beatmaps: None,
            pack_tags: None,
            converts: None,
            current_nominations: None,
            description: None,
            genre: None,
            language: None,
            ratings: Some([0, 2, 0, 0, 0, 0, 0, 0, 1, 0, 19]),
            recent_favourites: None,
            related_users: None,
            related_tags: None,
            user: None
        }),
        current_user_tag_ids: None,
        failtimes: Some(Failtimes {
            fail: [0, 0, 0, 0, 0, 45, 18, 18, 0, 9, 0, 0, 0, 9, 0, 0, 0, 36, 117, 18, 9, 72, 18, 18, 0, 0, 18, 27, 0, 0, 54, 9, 9, 18, 0, 9, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 9, 9, 0, 0, 0, 9, 18, 0, 0, 9, 9, 9, 9, 27, 18, 9, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            exit: [0, 0, 0, 0, 0, 0, 81, 27, 0, 36, 18, 9, 9, 36, 18, 9, 9, 9, 54, 27, 18, 27, 27, 9, 18, 18, 63, 9, 9, 0, 18, 9, 0, 9, 0, 18, 0, 0, 0, 0, 0, 0, 0, 9, 9, 9, 9, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }),
        owners: Some([Owner {
            id: 11119539,
            username: "AirinCat"
        }]),
        top_tag_ids: None
    }]
}
*/
