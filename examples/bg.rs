// Beatmap Get
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
    let beatmap = client.beatmaps.get_beatmap(5000433).await?;
    println!("{:?}", beatmap);

    Ok(())
}

/*
ReqwestBeatmaps get_Beatmap
Beatmap {
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
    passcount: 179,
    playcount: 996,
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
        play_count: 4336,
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
        fail: [0, 0, 0, 0, 0, 45, 18, 18, 0, 9, 0, 0, 0, 9, 0, 0, 0, 36, 108, 18, 9, 72, 18, 18, 0, 0, 18, 27, 0, 0, 54, 9, 9, 18, 0, 9, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 9, 9, 0, 0, 0, 9, 18, 0, 0, 9, 9, 9, 9, 27, 18, 9, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        exit: [0, 0, 0, 0, 0, 0, 72, 18, 0, 36, 18, 9, 9, 36, 18, 9, 9, 9, 54, 27, 18, 18, 27, 9, 9, 18, 63, 9, 9, 0, 18, 9, 0, 9, 0, 18, 0, 0, 0, 0, 0, 0, 0, 9, 9, 9, 9, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }),
    owners: Some([Owner {
        id: 11119539,
        username: "AirinCat"
    }]),
    top_tag_ids: None
}
*/
