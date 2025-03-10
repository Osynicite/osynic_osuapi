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
    let user_score = client
        .beatmaps
        .get_user_score(4222040, 31175842, None, Some(Mode::Catch), None)
        .await?;
    println!("{:?}", user_score);

    Ok(())
}

/*
ReqwestBeatmaps get_User_Score
BeatmapUserScore {
    position: Some(42),
    score: Some(Score {
        accuracy: 0.9922530664945126,
        best_id: 220051667,
        created_at: "2025-03-04T14:59:15Z",
        id: 220051667,
        max_combo: 688,
        mode: Catch,
        mode_int: 2,
        mods: [],
        passed: true,
        perfect: false,
        pp: 21.1261,
        rank: S,
        replay: true,
        score: 13010058,
        statistics: Statistics {
            count_100: 0,
            count_300: 1026,
            count_50: 511,
            count_geki: None,
            count_katu: 9,
            count_miss: 3
        },
        type: "score_best_fruits",
        user_id: 31175842,
        current_user_attributes: CurrentUserAttributes {
            pin: None
        },
        beatmap: Beatmap {
            beatmapset_id: 1985060,
            difficulty_rating: 1.73,
            id: 4222040,
            mode: Catch,
            status: Ranked,
            total_length: 219,
            user_id: 1634445,
            version: "dailycare's hard",
            accuracy: 6.2,
            ar: 8.0,
            bpm: 150.0,
            convert: true,
            count_circles: 301,
            count_sliders: 330,
            count_spinners: 2,
            cs: 3.7,
            deleted_at: None,
            drain: 4.8,
            hit_length: 195,
            is_scoreable: true,
            last_updated: "2023-07-31T13:48:41Z",
            mode_int: 2,
            passcount: 6245,
            playcount: 19270,
            ranked: 1,
            url: "https://osu.ppy.sh/beatmaps/4222040",
            checksum: "9e7f0204a1aab0d716f1f74e1dda326c",
            max_combo: None,
            beatmapset: None,
            current_user_tag_ids: None,
            failtimes: None,
            owners: Some([Owner {
                id: 1634445,
                username: "Dailycare"
            }]),
            top_tag_ids: None
        },
        user: User {
            avatar_url: "https://a.ppy.sh/31175842?1738717996.jpeg",
            country_code: "CN",
            default_group: "default",
            id: 31175842,
            is_active: true,
            is_bot: false,
            is_deleted: false,
            is_online: false,
            is_supporter: false,
            last_visit: "2025-03-07T04:39:32+00:00",
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
        }
    }),
    error: None
}

*/
