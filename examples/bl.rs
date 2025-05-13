// Beatmap Lookup
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::beatmaps::IBeatmaps;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let access_token = std::env
        ::var("ACCESS_TOKEN")
        .expect(
            "Please set the ACCESS_TOKEN environment variable to a valid osu! API v2 access token"
        );
    let client = OsynicOsuApiV2Client::new(OToken {
        access_token,
        refresh_token: None,
        expires_in: 86400,
        token_type: "Bearer".to_string(),
    });
    let beatmap = client.beatmaps.lookup(
        Some("69f77b0dfe67d288c1bf748f91ceb133".to_string()),
        None,
        None
    ).await?;
    println!("{:?}", beatmap);

    Ok(())
}

/*
ReqwestBeatmaps lookup
Beatmap {
	beatmapset_id: 12216,
	difficulty_rating: 2.5,
	id: 46004,
	mode: Osu,
	status: Ranked,
	total_length: 89,
	user_id: 2363,
	version: "Normal",
	accuracy: 4.0,
	ar: 4.0,
	bpm: 139.5,
	convert: false,
	count_circles: 75,
	count_sliders: 60,
	count_spinners: 1,
	cs: 4.0,
	deleted_at: None,
	drain: 3.0,
	hit_length: 81,
	is_scoreable: true,
	last_updated: "2014-05-18T17:11:23Z",
	mode_int: 0,
	passcount: 424640,
	playcount: 946724,
	ranked: 1,
	url: "https://osu.ppy.sh/beatmaps/46004",
	checksum: "69f77b0dfe67d288c1bf748f91ceb133",
	max_combo: Some(335),
	beatmapset: Some(Beatmapset {
		artist: "fripSide",
		artist_unicode: "fripSide",
		covers: Covers {
			cover: "https://assets.ppy.sh/beatmaps/12216/covers/cover.jpg?1622031333",
			cover_2x: "https://assets.ppy.sh/beatmaps/12216/covers/cover@2x.jpg?1622031333",
			card: "https://assets.ppy.sh/beatmaps/12216/covers/card.jpg?1622031333",
			card_2x: "https://assets.ppy.sh/beatmaps/12216/covers/card@2x.jpg?1622031333",
			list: "https://assets.ppy.sh/beatmaps/12216/covers/list.jpg?1622031333",
			list_2x: "https://assets.ppy.sh/beatmaps/12216/covers/list@2x.jpg?1622031333",
			slimcover: "https://assets.ppy.sh/beatmaps/12216/covers/slimcover.jpg?1622031333",
			slimcover_2x: "https://assets.ppy.sh/beatmaps/12216/covers/slimcover@2x.jpg?1622031333"
		},
		creator: "DJPop",
		favourite_count: 1316,
		hype: None,
		id: 12216,
		nsfw: false,
		offset: 0,
		play_count: 4170440,
		preview_url: "//b.ppy.sh/preview/12216.mp3",
		source: "To Aru Kagaku no Railgun",
		spotlight: false,
		status: Ranked,
		title: "LEVEL5 -judgelight- (TV Size)",
		title_unicode: "LEVEL5 -judgelight- (TV Size)",
		track_id: None,
		user_id: 2363,
		video: true,
		bpm: Some(139.5),
		can_be_hyped: Some(false),
		deleted_at: None,
		discussion_enabled: Some(true),
		discussion_locked: Some(false),
		is_scoreable: Some(true),
		last_updated: Some("2010-01-18T14:26:22Z"),
		legacy_thread_url: Some("https://osu.ppy.sh/community/forums/topics/22785"),
		nominations_summary: Some(NominationsSummary {
			current: 0,
			eligible_main_rulesets: ["osu"],
			required_meta: RequiredMeta {
				main_ruleset: 2,
				non_main_ruleset: 1
			}
		}),
		ranked: Some(1),
		ranked_date: Some("2010-01-18T17:26:31Z"),
		storyboard: Some(false),
		submitted_date: Some("2010-01-16T04:36:21Z"),
		tags: Some("とある科学の超電磁砲"),
		availability: Some(Availability {
			download_disabled: false,
			more_information: None
		}),
		beatmaps: None,
		pack_tags: None,
		converts: None,
		current_nominations: None,
		description: None,
		genre: None,
		language: None,
		ratings: Some([0, 171, 36, 41, 53, 105, 131, 267, 528, 1349, 6034]),
		recent_favourites: None,
		related_users: None,
		related_tags: None,
		user: None
	}),
	current_user_tag_ids: None,
	failtimes: Some(Failtimes {
		fail: [0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 20, 119, 960, 829, 990, 864, 1099, 1318, 1602, 2094, 2096, 1946, 1536, 1716, 1315, 1384, 1655, 1306, 1378, 1523, 1910, 1056, 1163, 871, 1359, 1546, 2084, 2146, 2669, 3517, 4561, 4202, 3069, 3877, 3711, 2265, 1207, 2007, 4364, 5899, 3388, 1924, 1238, 1375, 826, 606, 376, 676, 661, 919, 825, 696, 629, 349, 345, 446, 447, 746, 950, 939, 795, 616, 304, 402, 345, 399, 281, 317, 339, 276, 430, 1110, 1841, 2219, 1482, 1207, 1580, 3543, 1360, 2743, 3122, 3709, 2681, 1648, 3924, 7009, 5304, 5743, 12290, 12546],
		exit: [0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 9, 9, 0, 2787, 3175, 5471, 4766, 4285, 5214, 6350, 12131, 7302, 6183, 9299, 8766, 5132, 4763, 4588, 4172, 6670, 5246, 3731, 2406, 2541, 3662, 5409, 5461, 5906, 4862, 4722, 6422, 13175, 8712, 5384, 4609, 3988, 2547, 1508, 4423, 9050, 5949, 3259, 3105, 1786, 1837, 1233, 1292, 1226, 1635, 1310, 1562, 999, 757, 1434, 990, 1067, 1011, 1195, 1339, 1879, 1323, 1071, 1437, 1268, 957, 995, 870, 776, 949, 736, 860, 890, 3646, 4987, 3952, 1769, 1100, 3666, 3275, 1677, 3489, 1780, 3819, 1799, 838, 1995, 2453, 1850, 2003, 6722]
	}),
	owners: Some([Owner {
		id: 2363,
		username: "DJPop"
	}]),
	top_tag_ids: None
}
*/
