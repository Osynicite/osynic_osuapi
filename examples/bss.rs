use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::beatmapsets::IBeatmapsets;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::model::search::dtos::params::BeatmapsetsSearchParams;

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
    // let params = BeatmapsetsSearchParams::default().query("Bougainv543534534534534534334534illea".to_string());
    let params = BeatmapsetsSearchParams::default();
    let res = client.beatmapsets.search(params).await?;
    println!("{}", res.total);
    // println!("{:?}", res.beatmapsets);
    println!("{:?}", res.search);
    println!("{:?}", res.recommended_difficulty);
    println!("{:?}", res.error);
    println!("{:?}", res.cursor);
    println!("{:?}", res.cursor_string);
    // println!("{:?}", res.beatmapsets);
    // if let Some(beatmapsets) = res.beatmapsets {
    //     // for beatmapset in beatmapsets {
    //     //     println!("{:?}", beatmapset);
    //     // }
    //     // 只打印第一个
    //     println!("{:?}", beatmapsets[0]);
    // }

    println!("{:?}", res.beatmapsets[0]);
    Ok(())

    // ReqwestBeatmapsets search
    // Response { url: "https://osu.ppy.sh/api/v2/beatmapsets/search", status: 200, headers: {"date": "Fri, 07 Mar 2025 02:06:55 GMT", "content-type": "application/json", "transfer-encoding": "chunked", "connection": "keep-alive", "cache-control": "no-cache, private", "x-ratelimit-limit": "1200", "x-ratelimit-remaining": "1198", "strict-transport-security": "max-age=31536000; includeSubDomains; preload", "x-frame-options": "SAMEORIGIN", "vary": "accept-encoding", "cf-cache-status": "DYNAMIC", "x-content-type-options": "nosniff", "server": "cloudflare", "cf-ray": "91c6894a685fc8c3-HKG"} }
    // 48063
    // Search { sort: "ranked_desc" }
    // None
    // None
    // Some(Cursor { approved_date: 1741017900000, id: 2086784 })
    // Some("eyJhcHByb3ZlZF9kYXRlIjoxNzQxMDE3OTAwMDAwLCJpZCI6MjA4Njc4NH0")
    // Beatmapset { artist: "Maeda Jun x yanaginagi", artist_unicode: "麻枝 准×やなぎなぎ", covers: Covers { cover: "https://assets.ppy.sh/beatmaps/2315969/covers/cover.jpg?1740685143", cover_2x: "https://assets.ppy.sh/beatmaps/2315969/covers/cover@2x.jpg?1740685143", card: "https://assets.ppy.sh/beatmaps/2315969/covers/card.jpg?1740685143", card_2x: "https://assets.ppy.sh/beatmaps/2315969/covers/card@2x.jpg?1740685143", list: "https://assets.ppy.sh/beatmaps/2315969/covers/list.jpg?1740685143", list_2x: "https://assets.ppy.sh/beatmaps/2315969/covers/list@2x.jpg?1740685143", slimcover: "https://assets.ppy.sh/beatmaps/2315969/covers/slimcover.jpg?1740685143", slimcover_2x: "https://assets.ppy.sh/beatmaps/2315969/covers/slimcover@2x.jpg?1740685143" }, creator: "Djulus", favourite_count: 11, hype: None, id: 2315969, nsfw: false, offset: 0, play_count: 297, preview_url: "//b.ppy.sh/preview/2315969.mp3", source: "ヘブンバーンズレッド", spotlight: false, status: Ranked, title: "Bougainvillea", title_unicode: "Bougainvillea", track_id: None, user_id: 4960893, video: false, bpm: 145.5, can_be_hyped: false, deleted_at: None, discussion_enabled: true, discussion_locked: false, is_scoreable: true, last_updated: "2025-02-27T19:38:47Z", legacy_thread_url: "https://osu.ppy.sh/community/forums/topics/2034391", nominations_summary: NominationsSummary { current: 2, eligible_main_rulesets: ["osu"], required_meta: RequiredMeta { main_ruleset: 2, non_main_ruleset: 1 } }, ranked: 1, ranked_date: "2025-03-06T20:00:59Z", storyboard: false, submitted_date: "2025-01-30T07:37:51Z", tags: "yohanes japanese video game pop hbr heavens burn red visual arts key", availability: Availability { download_disabled: false, more_information: None }, beatmaps: [Beatmap { beatmapset_id: 2315969, difficulty_rating: 4.51, id: 4957881, mode: Osu, status: Ranked, total_length: 286, user_id: 4960893, version: "Collab", accuracy: 7.0, ar: 8.0, bpm: 145.5, convert: false, count_circles: 599, count_sliders: 325, count_spinners: 2, cs: 4.0, deleted_at: None, drain: 5.0, hit_length: 271, is_scoreable: true, last_updated: "2025-02-27T19:38:48Z", mode_int: 0, passcount: 32, playcount: 297, ranked: 1, url: "https://osu.ppy.sh/beatmaps/4957881", checksum: "869a7056da4492e85d597063289f8bdf", max_combo: 1287 }], pack_tags: [] }
}
